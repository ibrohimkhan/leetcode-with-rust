// https://leetcode.com/problems/kth-largest-element-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        for _ in 1..k { heap.pop(); }
        
        heap.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, 4);
    }
}
