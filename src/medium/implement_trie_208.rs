// https://leetcode.com/problems/implement-trie-prefix-tree/

use std::slice::Iter;

struct Trie {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {  
            
        }
    }
    
    fn insert(&self, word: String) {
    }
    
    fn search(&self, word: String) -> bool {
        true
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        true
    }
}

//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);