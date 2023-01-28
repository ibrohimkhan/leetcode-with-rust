// https://leetcode.com/problems/find-subarrays-with-equal-sum/

use crate::Solution;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                if nums[i] + nums[i + 1] == nums[j] + nums[j + 1] {
                    return true;
                }
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 2, 4];
        let result = Solution::find_subarrays(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::find_subarrays(nums);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 0, 0];
        let result = Solution::find_subarrays(nums);
        assert!(result);
    }
}
