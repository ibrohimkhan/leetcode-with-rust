// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut i = 2;
        
        while i < len {
            if nums[i - 2] == nums[i - 1] && nums[i - 1] == nums[i] {
                nums.remove(i);
                len -= 1;
                continue;
            }

            i += 1;
        }
        
        nums.len() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut nums = vec![1,1,1,2,2,3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0,0,1,1,1,1,2,3,3];
        let result = Solution::remove_duplicates(&mut nums);
        assert_eq!(result, 7);
    }
}