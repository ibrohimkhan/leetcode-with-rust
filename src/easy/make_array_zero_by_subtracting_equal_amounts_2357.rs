// https://leetcode.com/problems/make-array-zero-by-subtracting-equal-amounts/

use crate::Solution;

impl Solution {
    pub fn minimum_operations_2357(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<std::collections::HashSet<_>>();
        if set.contains(&0) {
            set.len() as i32 - 1
        } else {
            set.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 5, 0, 3, 5];
        let result = Solution::minimum_operations_2357(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        let result = Solution::minimum_operations_2357(nums);
        assert_eq!(result, 0);
    }
}
