#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fetcher;

use crate::fetcher::{CodeDefinition, Problem};
use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

use futures::executor::block_on;
use futures::executor::ThreadPool;
use futures::future::join_all;
use futures::stream::StreamExt;
use futures::task::SpawnExt;
use std::sync::{Arc, Mutex};

/// main() helps to generate the submission template .rs
fn main() {
    println!("Welcome to leetcode-rust system.\n");
    let mut initialized_ids = get_initialized_ids();
    loop {
        println!(
            "Please enter a frontend problem id, \n\
            or \"random\" to generate a random one, \n\
            or \"solve $i\" to move problem to solution/, \n\
            or \"all\" to initialize all problems \n"
        );
        let mut is_random = false;
        let mut is_solving = false;
        let mut id: u32 = 0;
        let mut id_arg = String::new();
        io::stdin()
            .read_line(&mut id_arg)
            .expect("Failed to read line");
        let id_arg = id_arg.trim();

        let random_pattern = Regex::new(r"^random$").unwrap();
        let solving_pattern = Regex::new(r"^solve (\d+)$").unwrap();
        let all_pattern = Regex::new(r"^all$").unwrap();

        if random_pattern.is_match(id_arg) {
            println!("You select random mode.");
            id = generate_random_id(&initialized_ids);
            is_random = true;
            println!("Generate random problem: {}", &id);
        } else if solving_pattern.is_match(id_arg) {
            // solve a problem
            // move it from problem/ to solution/
            is_solving = true;
            id = solving_pattern
                .captures(id_arg)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            deal_solving(&id);
            break;
        } else if all_pattern.is_match(id_arg) {
            // deal all problems
            let pool = ThreadPool::new().unwrap();
            let mut tasks = vec![];
            let problems = fetcher::get_problems().unwrap();
            let mut mod_file_addon = Arc::new(Mutex::new(vec![]));
            for problem_stat in problems.stat_status_pairs {
                if initialized_ids.contains(&problem_stat.stat.frontend_question_id) {
                    continue;
                }
                let mod_file_addon = mod_file_addon.clone();
                tasks.push(
                    pool.spawn_with_handle(async move {
                        let problem = fetcher::get_problem_async(problem_stat).await;
                        if problem.is_none() {
                            return;
                        }
                        let problem = problem.unwrap();
                        let code = problem
                            .code_definition
                            .iter()
                            .find(|&d| d.value == "rust".to_string());
                        if code.is_none() {
                            println!("Problem {} has no rust version.", problem.question_id);
                            return;
                        }
                        // not sure this can be async
                        async {
                            mod_file_addon.lock().unwrap().push(format!(
                                "mod p{:04}_{};",
                                problem.question_id,
                                problem.title_slug.replace("-", "_")
                            ));
                        }
                        .await;
                        let code = code.unwrap();
                        // not sure this can be async
                        // maybe should use async-std io
                        async { deal_problem(&problem, &code, false) }.await
                    })
                    .unwrap(),
                );
            }
            block_on(join_all(tasks));
            let mut lib_file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open("./src/problem/mod.rs")
                .unwrap();
            writeln!(lib_file, "{}", mod_file_addon.lock().unwrap().join("\n"));
            break;
        } else {
            id = id_arg
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("not a number: {}", id_arg));
            if initialized_ids.contains(&id) {
                println!("The problem you chose has been initialized in problem/");
                continue;
            }
        }

        let problem = fetcher::get_problem(id).unwrap_or_else(|| {
            panic!(
                "Error: failed to get problem #{} \
                 (The problem may be paid-only or may not be exist).",
                id
            )
        });
        let code = problem
            .code_definition
            .iter()
            .find(|&d| d.value == "rust".to_string());
        if code.is_none() {
            println!("Problem {} has no rust version.", &id);
            initialized_ids.push(problem.question_id);
            continue;
        }
        let code = code.unwrap();
        deal_problem(&problem, &code, true);
        break;
    }
}

fn generate_random_id(except_ids: &[u32]) -> u32 {
    use rand::Rng;
    use std::fs;
    let mut rng = rand::thread_rng();
    loop {
        let res: u32 = rng.gen_range(1, 1106);
        if !except_ids.contains(&res) {
            return res;
        }
        println!(
            "Generate a random num ({}), but it is invalid (the problem may have been solved \
             or may have no rust version). Regenerate..",
            res
        );
    }
}

fn get_initialized_ids() -> Vec<u32> {
    let content = fs::read_to_string("./src/problem/mod.rs").unwrap();
    let id_pattern = Regex::new(r"p(\d{4})_").unwrap();
    id_pattern
        .captures_iter(&content)
        .map(|x| x.get(1).unwrap().as_str().parse().unwrap())
        .collect()
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse crate::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse crate::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse crate::util::point::Point;")
    }
    extra_use_line
}

fn parse_problem_link(problem: &Problem) -> String {
    format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

fn parse_discuss_link(problem: &Problem) -> String {
    format!(
        "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
        problem.title_slug
    )
}

fn insert_return_in_code(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{[ \n]+}").unwrap();
    match return_type {
        "ListNode" => re
            .replace(&code, "{\n        Some(Box::new(ListNode::new(0)))\n    }")
            .to_string(),
        "ListNode[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "TreeNode" => re
            .replace(
                &code,
                "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }",
            )
            .to_string(),
        "boolean" => re.replace(&code, "{\n        false\n    }").to_string(),
        "character" => re.replace(&code, "{\n        '0'\n    }").to_string(),
        "character[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "double" => re.replace(&code, "{\n        0f64\n    }").to_string(),
        "double[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "int[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "integer" => re.replace(&code, "{\n        0\n    }").to_string(),
        "integer[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "integer[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<String>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<TreeNode>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<boolean>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<double>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<integer>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<list<integer>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<list<string>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "list<string>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "null" => code.to_string(),
        "string" => re
            .replace(&code, "{\n        String::new()\n    }")
            .to_string(),
        "string[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
        "void" => code.to_string(),
        "NestedInteger" => code.to_string(),
        "Node" => code.to_string(),
        _ => code.to_string(),
    }
}

fn build_desc(content: &str) -> String {
    // TODO: fix this shit
    content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ")
}

fn deal_solving(id: &u32) {
    let problem = fetcher::get_problem(*id).unwrap();
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    // check problem/ existence
    if !file_path.exists() {
        panic!("problem does not exist");
    }
    // check solution/ no existence
    let solution_name = format!(
        "s{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    let solution_path = Path::new("./src/solution").join(format!("{}.rs", solution_name));
    if solution_path.exists() {
        panic!("solution exists");
    }
    // rename/move file
    fs::rename(file_path, solution_path).unwrap();
    // remove from problem/mod.rs
    let mod_file = "./src/problem/mod.rs";
    let target_line = format!("mod {};", file_name);
    let lines: Vec<String> = io::BufReader::new(File::open(mod_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| *x != target_line)
        .collect();
    fs::write(mod_file, lines.join("\n"));
    // insert into solution/mod.rs
    let mut lib_file = fs::OpenOptions::new()
        .append(true)
        .open("./src/solution/mod.rs")
        .unwrap();
    writeln!(lib_file, "mod {};", solution_name);
}

fn deal_problem(problem: &Problem, code: &CodeDefinition, write_mod_file: bool) {
    let file_name = format!(
        "p{:04}_{}",
        problem.question_id,
        problem.title_slug.replace("-", "_")
    );
    let file_path = Path::new("./src/problem").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let template = fs::read_to_string("./template.rs").unwrap();
    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_return_in_code(&problem.return_type, &code.default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
        .replace("__PROBLEM_LINK__", &parse_problem_link(problem))
        .replace("__DISCUSS_LINK__", &parse_discuss_link(problem));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    if write_mod_file {
        let mut lib_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("./src/problem/mod.rs")
            .unwrap();
        writeln!(lib_file, "mod {};", file_name);
    }
}
