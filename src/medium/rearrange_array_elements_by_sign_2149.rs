// https://leetcode.com/problems/rearrange-array-elements-by-sign/

use crate::Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let pos = nums.iter().filter(|item| item.is_positive());
        let neg = nums.iter().filter(|item| item.is_negative());

        pos.zip(neg).flat_map(|(&a, &b)| vec![a, b]).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 1, -2, -5, 2, -4];
        let result = Solution::rearrange_array(nums);
        assert_eq!(result, vec![3, -2, 1, -5, 2, -4]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 1];
        let result = Solution::rearrange_array(nums);
        assert_eq!(result, vec![1, -1]);
    }
}
