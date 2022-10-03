// https://leetcode.com/problems/wiggle-sort/

use crate::Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        for i in 0..nums.len() - 1  {
            if (i % 2 == 0) == (nums[i] > nums[i + 1]) {
                nums.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 5, 2, 1, 6, 4];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![3, 5, 1, 6, 2, 4]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![6, 6, 5, 6, 3, 8];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![6, 6, 5, 6, 3, 8]);
    }
}
