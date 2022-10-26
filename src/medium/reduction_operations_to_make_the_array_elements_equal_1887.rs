// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/

use crate::Solution;

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        let (mut result, len) = (0, nums.len());
        nums.sort();

        for i in (1..=len - 1).rev() {
            if nums[i] != nums[i - 1] {
                result += len - i;
            }
        }

        result as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![5, 1, 3];
        let result = Solution::reduction_operations(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1];
        let result = Solution::reduction_operations(nums);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 1, 2, 2, 3];
        let result = Solution::reduction_operations(nums);
        assert_eq!(result, 4);
    }
}
