// https://leetcode.com/problems/majority-element/

use crate::Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(nums), 2);
    }
}
