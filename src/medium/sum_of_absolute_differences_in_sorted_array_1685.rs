// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/

use crate::Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let (mut left_sum, mut right_sum) = (0, nums.iter().sum::<i32>());
        let (n, mut result) = (nums.len(), vec![]);

        for (i, &num) in nums.iter().enumerate() {
            result.push(
                i as i32 * num - left_sum + right_sum - (n - i) as i32 * num
            );

            left_sum += num;
            right_sum -= num;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 5];
        let result = Solution::get_sum_absolute_differences(nums);
        assert_eq!(result, vec![4, 3, 5]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 4, 6, 8, 10];
        let result = Solution::get_sum_absolute_differences(nums);
        assert_eq!(result, vec![24, 15, 13, 15, 21]);
    }
}
