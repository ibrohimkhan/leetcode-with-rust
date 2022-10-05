// https://leetcode.com/problems/search-in-rotated-sorted-array/description/

use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        
        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;

            } else if nums[left] <= nums[mid] {
                if nums[left] <= target && target < nums[mid] { right = mid - 1; }
                else { left = mid + 1 };

            } else {
                if nums[right] >= target && target > nums[mid] { left = mid + 1; }
                else { right = mid - 1; }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let target = 0;
        let result = Solution::search(nums, target);
        assert_eq!(result, -1);
    }
}
