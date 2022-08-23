// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

use crate::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];

        if nums.is_empty() { return result; }
        if nums[0] > target || target > nums[nums.len() - 1] { return result; }
        
        let mut found_start_position = false;
        for i in 0..nums.len() {
            if nums[i] == target && !found_start_position {
                found_start_position = true;
                result[0] = i as i32;
                result[1] = i as i32;
                
            } else if nums[i] == target && found_start_position {
                result[1] = i as i32;
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
        let nums = vec![5,7,7,8,8,10];
        let target = 8;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_2() {
        let nums = vec![5,7,7,8,8,10];
        let target = 6;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![];
        let target = 0;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn test_4() {
        let nums = vec![1];
        let target = 1;
        let result = Solution::search_range(nums, target);
        assert_eq!(result, vec![0, 0]);
    }
}
