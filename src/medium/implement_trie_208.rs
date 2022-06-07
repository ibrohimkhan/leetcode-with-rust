// https://leetcode.com/problems/implement-trie-prefix-tree/

const ASCII_A: usize = 97;

// converts ASCII char into usize, such that 'a' -> 0, 'b' -> 1, ..., 'z' -> 25
fn to_index(ch: char) -> usize {
    ch as usize - ASCII_A
}

#[derive(Debug, Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;

        for index in word.chars().map(to_index) {
            node = node.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }

        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(word).map_or(false, |trie| trie.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: String) -> Option<&Trie> {
        let mut node = self;

        for index in word.chars().map(to_index) {
            match &node.children[index] {
                Some(new_node) => node = new_node,
                None => return None,
            }
        }

        Some(node)
    }
}

//  * Your Trie object will be instantiated and called as such:
//  * let obj = Trie::new();
//  * obj.insert(word);
//  * let ret_2: bool = obj.search(word);
//  * let ret_3: bool = obj.starts_with(prefix);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = Trie::new();
        obj.insert("word".to_string());

        let ret_2 = obj.search("word".to_string());
        assert_eq!(ret_2, true);

        let ret_3 = obj.starts_with("w".to_string());
        assert_eq!(ret_3, true);

        let ret_4 = obj.starts_with("wi".to_string());
        assert_eq!(ret_4, false);
    }
}
