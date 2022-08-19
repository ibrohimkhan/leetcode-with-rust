// https://leetcode.com/problems/single-number/

use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in nums {
            result ^= i;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 2, 1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let result = Solution::single_number(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let result = Solution::single_number(nums);
        assert_eq!(result, 1);
    }
} 