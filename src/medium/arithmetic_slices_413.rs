// https://leetcode.com/problems/arithmetic-slices/

use crate::Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut count = 0;
        
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                count += 1;
                sum += count;
            } else {
                count = 0;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::number_of_arithmetic_slices(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let result = Solution::number_of_arithmetic_slices(nums);
        assert_eq!(result, 0);
    }
}
