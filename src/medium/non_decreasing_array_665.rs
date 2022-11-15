// https://leetcode.com/problems/non-decreasing-array/

use crate::Solution;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut modified = false;

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if modified { return false; }
                modified = true;

                if i >= 2
                    && i < nums.len() - 1
                    && nums[i - 2] > nums[i]
                    && nums[i - 1] > nums[i + 1]
                {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 2, 3];
        let result = Solution::check_possibility(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 2, 1];
        let result = Solution::check_possibility(nums);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 4, 2, 3];
        let result = Solution::check_possibility(nums);
        assert_eq!(result, false);
    }
}
