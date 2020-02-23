/**
 * [211] Add and Search Word - Data structure design
 *
 * Design a data structure that supports the following two operations:
 *
 *
 * void addWord(word)
 * bool search(word)
 *
 *
 * search(word) can search a literal word or a regular expression string containing only letters a-z or .. A . means it can represent any one letter.
 *
 * Example:
 *
 *
 * addWord("bad")
 * addWord("dad")
 * addWord("mad")
 * search("pad") -> false
 * search("bad") -> true
 * search(".ad") -> true
 * search("b..") -> true
 *
 *
 * Note:<br />
 * You may assume that all words are consist of lowercase letters a-z.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/add-and-search-word-data-structure-design/
// discuss: https://leetcode.com/problems/add-and-search-word-data-structure-design/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct WordDictionary {
    root: Option<Box<Trie>>,
}

#[derive(Default)]
struct Trie {
    is_end: bool,
    marked: Vec<usize>,
    nodes: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    // /** Initialize your data structure here. */
    // fn new() -> Self {
    //     WordDictionary{
    //         root: Some(Box::new(Trie::new())),
    //     }
    // }

    // /** Adds a word into the data structure. */
    // fn add_word(&mut self, word: String) {
    //     let mut curr = self.root;

    //     for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
    //         curr = curr.as_ref().unwrap().nodes[i];
    //     }
    //     curr.as_mut().unwrap().is_end = true;
    // }

    // /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    // fn search(&self, word: String) -> bool {
    //     let mut chs: Vec<char> = word.chars().collect();
    //     WordDictionary::search_from(self.root.as_ref(), &mut chs)
    // }

    // fn search_from(node: Option<&Box<Trie>>, chs: &mut [char]) -> bool {
    //     if node.is_none() {
    //         return false
    //     }
    //     let node = node.unwrap();
    //     if chs.len() < 1 {
    //         return node.is_end
    //     }
    //     if chs[0] == '.' {
    //         // backtracking
    //         let mut sliced = &mut chs[1..];
    //         for &idx in node.marked.iter() {
    //             if WordDictionary::search_from(node.nodes[idx].as_ref(), sliced) {
    //                 return true
    //             }
    //         }
    //         false
    //     } else {
    //         let next = node.nodes[(chs[0] as u8 - 'a' as u8) as usize].as_ref();
    //         WordDictionary::search_from(next, &mut chs[1..])
    //     }
    // }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_211() {
        // let mut dict = WordDictionary::new();
        // dict.add_word("bad".to_owned());
        // dict.add_word("dad".to_owned());
        // dict.add_word("mad".to_owned());
        // assert_eq!(dict.search("pad".to_owned()), false);
        // assert_eq!(dict.search("bad".to_owned()), true);
        // assert_eq!(dict.search(".ad".to_owned()), true);
        // assert_eq!(dict.search("da.".to_owned()), true);
    }
}
