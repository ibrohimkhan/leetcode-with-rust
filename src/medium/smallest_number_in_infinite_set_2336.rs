// https://leetcode.com/problems/smallest-number-in-infinite-set/

use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

struct SmallestInfiniteSet {
    p_queue: BinaryHeap<Reverse<i32>>,
    seen: HashSet<i32>, 
}

impl SmallestInfiniteSet {

    fn new() -> Self {
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();

        for i in 1..1001 {
            heap.push(Reverse(i));
            set.insert(i);
        }

        Self { p_queue: heap, seen: set }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(item)) = self.p_queue.pop() {
            self.seen.remove(&item);
            item
        } else { -1 }
    }
    
    fn add_back(&mut self, num: i32) {
        if self.seen.insert(num) {
            self.p_queue.push(Reverse(num));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn test() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);

        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);
        assert_eq!(set.pop_smallest(), 3);
        
        set.add_back(1);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 4);
        assert_eq!(set.pop_smallest(), 5);
    }
}
