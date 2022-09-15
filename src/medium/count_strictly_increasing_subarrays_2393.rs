// https://leetcode.com/problems/count-strictly-increasing-subarrays/

use crate::Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i64 {
        let mut dp = vec![0; nums.len()];
        dp[0] = 1;

        let mut result = 1i64;
        for i in 1..nums.len() {
            dp[i] = if nums[i] > nums[i - 1] { 1 + dp[i - 1] } else { 1 };
            result += dp[i]; 
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 5, 4, 4, 6];
        let result = Solution::count_subarrays(nums);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::count_subarrays(nums);
        assert_eq!(result, 15);
    }
}
