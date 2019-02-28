/**
 * [146] LRU Cache
 *
 * 
 * Design and implement a data structure for <a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>. It should support the following operations: get and put.
 * 
 * 
 * 
 * get(key) - Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.<br>
 * put(key, value) - Set or insert the value if the key is not already present. When the cache reached its capacity, it should invalidate the least recently used item before inserting a new item.
 * 
 * 
 * Follow up:<br />
 * Could you do both operations in O(1) time complexity?
 * 
 * Example:
 * 
 * LRUCache cache = new LRUCache( 2 /* capacity */ );
 * 
 * cache.put(1, 1);
 * cache.put(2, 2);
 * cache.get(1);       // returns 1
 * cache.put(3, 3);    // evicts key 2
 * cache.get(2);       // returns -1 (not found)
 * cache.put(4, 4);    // evicts key 1
 * cache.get(1);       // returns -1 (not found)
 * cache.get(3);       // returns 3
 * cache.get(4);       // returns 4
 * 
 * 
 */

// submission codes start here

/*
 Least Recently Used, 最近最少使用, 关键在于追踪每一个 entry 的 age, 每次淘汰最小的那一个 key

 假如淘汰逻辑要做到 O(1) 复杂度, 我们可以引入一个链表, 每次 touch 一个值时, 就删掉它重新 push_back, 而当达到容量要驱逐时, 则 pop_front

 Rust 的链表不支持根据引用删除任意元素，这个工程实现还是有点挑战的, 晚点再做
 */
use std::collections::HashMap;
use std::collections::LinkedList;
struct LRUCache {
    cache: HashMap<i32, i32>,
    vec: Vec<i32>,
    cap: i32,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache{
            cache: HashMap::with_capacity(capacity as usize,),
            vec: Vec::with_capacity(capacity as usize),
            cap: capacity,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let cache = HashMap::new();
        let list = Vec::new();
    }
    
    fn put(&mut self, key: i32, value: i32) {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_146() {
    }
}
