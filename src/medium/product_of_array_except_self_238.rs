// https://leetcode.com/problems/product-of-array-except-self/

use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut result = vec![1; len];

        for i in 1..len {
            result[i] = nums[i - 1] * result[i - 1];
        }

        let mut temp = 1;
        for i in (0..=len - 1).rev() {
            result[i] = result[i] * temp;
            temp *= nums[i]
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, vec![0, 0, 9, 0, 0]);
    }
}
