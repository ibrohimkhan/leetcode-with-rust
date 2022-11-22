// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/

use crate::Solution;

impl Solution {
    pub fn max_product_1464(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        (nums.pop().unwrap() - 1) * (nums.pop().unwrap() - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 2];
        let result = Solution::max_product_1464(nums);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 5, 4, 5];
        let result = Solution::max_product_1464(nums);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 7];
        let result = Solution::max_product_1464(nums);
        assert_eq!(result, 12);
    }
}
