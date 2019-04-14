/**
 * [208] Implement Trie (Prefix Tree)
 *
 * Implement a trie with insert, search, and startsWith methods.
 *
 * Example:
 *
 *
 * Trie trie = new Trie();
 *
 * trie.insert("apple");
 * trie.search("apple");   // returns true
 * trie.search("app");     // returns false
 * trie.startsWith("app"); // returns true
 * trie.insert("app");
 * trie.search("app");     // returns true
 *
 *
 * Note:
 *
 *
 * 	You may assume that all inputs are consist of lowercase letters a-z.
 * 	All inputs are guaranteed to be non-empty strings.
 *
 *
 */
pub struct Solution {}

// submission codes start here

// use idx 27 as special end character
use std::cell::RefCell;
use std::rc::Rc;
struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct TrieNode {
    value: char,
    nodes: Vec<Option<Rc<RefCell<TrieNode>>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            root: Trie::new_node(' '),
        }
    }

    /** insert a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut curr = self.root.clone();
        for ch in word.chars() {
            let idx = Trie::to_idx(ch);
            if let Some(node) = curr.clone().borrow().nodes[idx].clone() {
                curr = node;
                continue;
            }
            let next = Some(Trie::new_node(ch));
            curr.borrow_mut().nodes[idx] = next.clone();
            curr = next.clone().unwrap();
        }
        // Add end char
        curr.borrow_mut().nodes[26] = Some(Trie::new_node(' '));
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut curr = self.root.clone();
        for ch in word.chars() {
            let idx = Trie::to_idx(ch);
            if let Some(node) = curr.clone().borrow().nodes[idx].clone() {
                curr = node;
            } else {
                return false;
            }
        }
        let searched = curr.borrow().nodes[26].is_some();
        searched
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self.root.clone();
        for ch in prefix.chars() {
            let idx = Trie::to_idx(ch);
            if let Some(node) = curr.clone().borrow().nodes[idx].clone() {
                curr = node;
            } else {
                return false;
            }
        }
        true
    }

    fn to_idx(ch: char) -> usize {
        (ch as u8 - 'a' as u8) as usize
    }

    fn new_node(ch: char) -> Rc<RefCell<TrieNode>> {
        Rc::new(RefCell::new(TrieNode {
            value: ch,
            nodes: vec![None; 27],
        }))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_208() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert_eq!(trie.search("apple".to_owned()), true); // returns true
        assert_eq!(trie.search("app".to_owned()), false);
        assert_eq!(trie.starts_with("app".to_owned()), true); // returns true
        trie.insert("app".to_owned());
        assert_eq!(trie.search("app".to_owned()), true); // returns true
    }
}
