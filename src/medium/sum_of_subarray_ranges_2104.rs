// https://leetcode.com/problems/sum-of-subarray-ranges/

use crate::Solution;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let mut result = 0i64;

        for i in 0..nums.len() {
            let (mut min, mut max) = (nums[i], nums[i]);
            for j in i..nums.len() {
                min = min.min(nums[j]);
                max = max.max(nums[j]);
                result += (max - min) as i64;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let result = Solution::sub_array_ranges(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 3];
        let result = Solution::sub_array_ranges(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![4, -2, -3, 4, 1];
        let result = Solution::sub_array_ranges(nums);
        assert_eq!(result, 59);
    }
}