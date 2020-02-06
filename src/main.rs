#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod fetcher;

use crate::fetcher::Problems;
use crate::fetcher::StatWithStatus;
use crate::fetcher::{CodeDefinition, Problem};
use futures::executor::block_on;
use futures::executor::ThreadPool;
use futures::future::join_all;
use futures::task::SpawnExt;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;

const PROBLEM_FOLDER: &str = "./src/problem";
const PROBLEM_PREFIX: &str = "p";
const SOLUTION_FOLDER: &str = "./src/solution";
const SOLUTION_PREFIX: &str = "s";
const MOD_FILE: &str = "/mod.rs";
const TEMPLATE_FILE: &str = "./template.rs";

/// main() helps to generate the submission template .rs
fn main() {
    println!("Welcome to leetcode-rust system.");
    let mut initialized_ids = get_initialized_ids();
    loop {
        println!("Please enter a frontend problem id, or \"random\" to generate a random one, or \"solve $i\" to move problem to solution/");
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
        } else if all_pattern.is_match(id_arg) {
            println!("You've chosen to generate all problems, which may takes a while...");
            let mut pool = ThreadPool::new().unwrap();
            let mut res = vec![];
            let problems = fetcher::get_problems().unwrap();
            for problem_stat in problems.stat_status_pairs {
                if !initialized_ids.contains(&problem_stat.stat.frontend_question_id) {
                    res.push(pool.spawn_with_handle(deal_problem(problem_stat)).unwrap());
                }
            }
            block_on(join_all(res));
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

        let problem =
            fetcher::get_problem(&get_problem_stat_map(fetcher::get_problems().unwrap())[&id])
                .unwrap_or_else(|| {
                    panic!(
                        "Error: failed to get problem #{} (The problem may be paid-only or may not be exist).",
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

        let file_name = get_file_name(&problem, PROBLEM_PREFIX);
        let file_path = Path::new(PROBLEM_FOLDER).join(format!("{}.rs", file_name));
        if is_solving {
            // check problem/ existence
            if !file_path.exists() {
                print!("problem does not exist");
                continue;
            }
            // check solution/ no existence
            let solution_name = get_file_name(&problem, SOLUTION_PREFIX);
            let solution_path = Path::new(SOLUTION_FOLDER).join(format!("{}.rs", solution_name));
            if solution_path.exists() {
                print!("solution exists");
                continue;
            }
            // rename/move file
            fs::rename(file_path, solution_path).unwrap();
            // remove from problem/mod.rs
            let mod_file = PROBLEM_FOLDER.to_owned() + MOD_FILE;
            let target_line = format!("mod {};", file_name);
            let lines: Vec<String> = io::BufReader::new(File::open(&mod_file).unwrap())
                .lines()
                .map(|x| x.unwrap())
                .filter(|x| *x != target_line)
                .collect();
            fs::write(mod_file, lines.join("\n"));
            // insert into solution/mod.rs
            let mut lib_file = fs::OpenOptions::new()
                .append(true)
                .open(SOLUTION_FOLDER.to_owned() + MOD_FILE)
                .unwrap();
            writeln!(lib_file, "mod {};", solution_name);
            break;
        }
        if file_path.exists() {
            panic!("problem already initialized");
        }

        let template = read_template();
        let source = parse_template(template, &problem, code);

        let mut file = open_problem_file(&file_path);

        write_problem_file(&mut file, source);
        drop(file);

        let mut lib_file = open_problem_lib_file();
        write_problem_lib_file(&mut lib_file, file_name);
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
    let content = fs::read_to_string(PROBLEM_FOLDER.to_owned() + MOD_FILE).unwrap();
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

fn insert_return_in_code(return_type: &str, code: &str) -> String {
    let re = Regex::new(r"\{[ \n\r]+}").unwrap();
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

async fn deal_problem(problem_stat: StatWithStatus) {
    let problem = async { fetcher::get_problem(&problem_stat) }.await;
    if problem.is_none() {
        println!(
            "Problem {} may be paid-only, or some error happens",
            &problem_stat.stat.frontend_question_id
        );
        return;
    }
    let problem = problem.unwrap();
    let code = problem
        .code_definition
        .iter()
        .find(|&d| d.value == "rust".to_string());
    if code.is_none() {
        println!(
            "Problem {} has no rust version.",
            &problem_stat.stat.frontend_question_id
        );
        return;
    }
    let code = code.unwrap();

    let file_name = get_file_name(&problem, PROBLEM_PREFIX);
    let file_path = Path::new(PROBLEM_FOLDER).join(format!("{}.rs", file_name));

    let template = async { read_template() }.await;
    let source = parse_template(template, &problem, code);

    let mut file = async { open_problem_file(&file_path) }.await;

    async { write_problem_file(&mut file, source) }.await;

    let mut lib_file = async { open_problem_lib_file() }.await;
    async { write_problem_lib_file(&mut lib_file, file_name) }.await;
    println!(
        "problem: {}.{} initialized",
        problem_stat.stat.frontend_question_id, problem.title_slug
    );
}

pub fn get_problem_stat_map(problems: Problems) -> HashMap<u32, StatWithStatus> {
    let mut ret = HashMap::new();
    for problem_stat in problems.stat_status_pairs {
        ret.insert(problem_stat.stat.frontend_question_id, problem_stat);
    }
    ret
}

// extract common code
pub fn get_file_name(problem: &Problem, prefix: &str) -> String {
    format!(
        "{}{:04}_{}",
        prefix,
        problem.question_id,
        problem.title_slug.replace("-", "_")
    )
}

pub fn read_template() -> String {
    fs::read_to_string(TEMPLATE_FILE).unwrap()
}

pub fn parse_template(template: String, problem: &Problem, code: &CodeDefinition) -> String {
    template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &build_desc(&problem.content))
        .replace(
            "__PROBLEM_DEFAULT_CODE__",
            &insert_return_in_code(&problem.return_type, &code.default_code),
        )
        .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code))
}

pub fn open_problem_file(file_path: &PathBuf) -> File {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap()
}

pub fn write_problem_file(file: &mut File, source: String) {
    file.write_all(source.as_bytes()).unwrap()
}

pub fn open_problem_lib_file() -> File {
    fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(PROBLEM_FOLDER.to_owned() + MOD_FILE)
        .unwrap()
}

pub fn write_problem_lib_file(lib_file: &mut File, file_name: String) {
    writeln!(lib_file, "mod {};", file_name);
}
