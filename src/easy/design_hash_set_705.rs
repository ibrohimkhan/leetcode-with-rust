// https://leetcode.com/problems/design-hashset/

struct MyHashSet {
    items: Vec<bool>
}

impl MyHashSet {

    fn new() -> Self {
        Self { items: vec![false; 1000001] }
    }
    
    fn add(&mut self, key: i32) {
        self.items[key as usize] = true;
    }
    
    fn remove(&mut self, key: i32) {
        self.items[key as usize] = false; 
    }
    
    fn contains(&self, key: i32) -> bool {
        self.items[key as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn test() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert_eq!(set.contains(1), true);
        assert_eq!(set.contains(3), false);

        set.add(2);
        assert_eq!(set.contains(2), true);
        
        set.remove(2);
        assert_eq!(set.contains(2), false);
    }
}
