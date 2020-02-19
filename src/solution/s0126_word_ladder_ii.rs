/**
 * [126] Word Ladder II
 *
 * Given two words (beginWord and endWord), and a dictionary's word list, find all shortest transformation sequence(s) from beginWord to endWord, such that:
 *
 * <ol>
 * 	Only one letter can be changed at a time
 * 	Each transformed word must exist in the word list. Note that beginWord is not a transformed word.
 * </ol>
 *
 * Note:
 *
 *
 * 	Return an empty list if there is no such transformation sequence.
 * 	All words have the same length.
 * 	All words contain only lowercase alphabetic characters.
 * 	You may assume no duplicates in the word list.
 * 	You may assume beginWord and endWord are non-empty and are not the same.
 *
 *
 * Example 1:
 *
 *
 * Input:
 * beginWord = "hit",
 * endWord = "cog",
 * wordList = ["hot","dot","dog","lot","log","cog"]
 *
 * Output:
 * [
 *   ["hit","hot","dot","dog","cog"],
 *   ["hit","hot","lot","log","cog"]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input:
 * beginWord = "hit"
 * endWord = "cog"
 * wordList = ["hot","dot","dog","lot","log"]
 *
 * Output: []
 *
 * Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
 *
 *
 *
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-ladder-ii/
// discuss: https://leetcode.com/problems/word-ladder-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
假如 A 经过一个字符的变换能得到 B, 则认为 A 与 B 之间有通路, 转化为一个 BFS 找无权图最短路径的问题

实现时, 可以先把图构造出来, 复杂度 O(L*N^2) (L 是字符串长度), 也可以每次都回到数组里去找连通点, 时间复杂度不变

由于要记录所有的路径, 因此我们需要把每个点的可能前置节点都记录下来, 最后用一个 DFS 或 BFS 找出所有路径

暂时想不到更好的办法
*/

use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        let len = word_list.len();
        let target = word_list.iter().position(|s| s == &end_word);
        if target.is_none() {
            return res;
        }
        let target = target.unwrap();
        let mut deq = VecDeque::new();
        deq.push_back(target);
        // paths record the distance & previous index, we use 'len' to represent empty prev
        let mut paths: Vec<(i32, Vec<usize>)> = vec![(i32::max_value(), vec![]); len];
        paths[target].0 = 0;
        let mut find_shortest = false;
        let mut shortest = i32::max_value();
        let mut in_queue = HashSet::new();
        while let Some(i) = deq.pop_front() {
            if Solution::connect(&begin_word, &word_list[i]) {
                // complete the path using dfs
                if paths[i].0 > shortest {
                    continue;
                }
                Solution::dfs(i, vec![begin_word.clone()], &word_list, &paths, &mut res);
                shortest = paths[i].0;
                find_shortest = true;
            }
            // we have found the shortest path, just drain all the nodes in deq
            if find_shortest {
                continue;
            }
            for j in 0..len {
                if j == i {
                    continue;
                }
                if Solution::connect(&word_list[i], &word_list[j]) {
                    if paths[i].0 + 1 <= paths[j].0 {
                        let mut prev = &mut paths[j].1;
                        prev.push(i);
                        paths[j].0 = paths[i].0 + 1;
                        if !in_queue.contains(&j) {
                            deq.push_back(j);
                            in_queue.insert(j);
                        }
                    }
                }
            }
        }
        res
    }

    fn dfs(
        curr: usize,
        mut path: Vec<String>,
        words: &Vec<String>,
        paths: &Vec<(i32, Vec<usize>)>,
        res: &mut Vec<Vec<String>>,
    ) {
        path.push(words[curr].clone());
        if paths[curr].1.is_empty() {
            res.push(path);
            return;
        }
        for &prev in paths[curr].1.iter() {
            Solution::dfs(prev, path.clone(), words, paths, res);
        }
    }

    #[inline(always)]
    fn connect(s1: &str, s2: &str) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut iter1 = s1.chars().into_iter();
        let mut iter2 = s2.chars().into_iter();
        let mut diff = 0;
        while let (Some(c1), Some(c2)) = (iter1.next(), iter2.next()) {
            if c1 != c2 {
                diff += 1;
                if diff >= 2 {
                    return false;
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_126() {
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                vec_string!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            vec![
                vec_string!["hit", "hot", "dot", "dog", "cog"],
                vec_string!["hit", "hot", "lot", "log", "cog"],
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                "cet".to_owned(),
                "ism".to_owned(),
                vec_string![
                    "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip",
                    "kay", "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die",
                    "war", "hay", "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot",
                    "bid", "ali", "pay", "col", "gum", "ger", "row", "won", "dan", "rum", "fad",
                    "tut", "sag", "yip", "sui", "ark", "has", "zip", "fez", "own", "ump", "dis",
                    "ads", "max", "jaw", "out", "btu", "ana", "gap", "cry", "led", "abe", "box",
                    "ore", "pig", "fie", "toy", "fat", "cal", "lie", "noh", "sew", "ono", "tam",
                    "flu", "mgm", "ply", "awe", "pry", "tit", "tie", "yet", "too", "tax", "jim",
                    "san", "pan", "map", "ski", "ova", "wed", "non", "wac", "nut", "why", "bye",
                    "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log", "tod", "dot",
                    "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib", "mel",
                    "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom",
                    "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit",
                    "doe", "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron",
                    "soy", "gin", "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop",
                    "but", "orb", "fen", "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod",
                    "yam", "pew", "web", "hod", "hun", "gyp", "wei", "wis", "rob", "gad", "pie",
                    "mon", "dog", "bib", "rub", "ere", "dig", "era", "cat", "fox", "bee", "mod",
                    "day", "apr", "vie", "nev", "jam", "pam", "new", "aye", "ani", "and", "ibm",
                    "yap", "can", "pyx", "tar", "kin", "fog", "hum", "pip", "cup", "dye", "lyx",
                    "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad", "ats", "set", "qom",
                    "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao", "mom", "mas",
                    "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may", "shy",
                    "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava",
                    "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd",
                    "you", "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw",
                    "nix", "ate", "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any",
                    "dow", "cod", "bed", "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao",
                    "aug", "mum", "hoe", "bah", "hal", "keg", "hew", "zed", "tow", "gog", "ass",
                    "dem", "who", "bet", "gos", "son", "ear", "spy", "kit", "boy", "due", "sen",
                    "oaf", "mix", "hep", "fur", "ada", "bin", "nil", "mia", "ewe", "hit", "fix",
                    "sad", "rib", "eye", "hop", "haw", "wax", "mid", "tad", "ken", "wad", "rye",
                    "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin", "mad", "ray", "hon",
                    "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay", "poi", "yep",
                    "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee", "wig",
                    "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit",
                    "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet",
                    "yaw", "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw",
                    "eel", "dix", "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt",
                    "lid", "pun", "ton", "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil",
                    "jig", "hub", "low", "did", "tin", "get", "gte", "sox", "lei", "mig", "fig",
                    "lon", "use", "ban", "flo", "nov", "jut", "bag", "mir", "sty", "lap", "two",
                    "ins", "con", "ant", "net", "tux", "ode", "stu", "mug", "cad", "nap", "gun",
                    "fop", "tot", "sow", "sal", "sic", "ted", "wot", "del", "imp", "cob", "way",
                    "ann", "tan", "mci", "job", "wet", "ism", "err", "him", "all", "pad", "hah",
                    "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com", "ill", "was",
                    "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran", "lab",
                    "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
                    "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig",
                    "cid", "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref",
                    "aha", "fib", "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo",
                    "win", "tao", "coy", "van", "man", "pit", "guy", "foe", "hid", "mai", "sup",
                    "jay", "hob", "mow", "jot", "are", "pol", "arc", "lax", "aft", "alb", "len",
                    "air", "pug", "pox", "vow", "got", "meg", "zoe", "amp", "ale", "bud", "gee",
                    "pin", "dun", "pat", "ten", "mob"
                ]
            ),
            vec![
                vec_string![
                    "cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism"
                ],
                vec_string![
                    "cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"
                ],
                vec_string![
                    "cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"
                ],
            ]
        );
    }
}
