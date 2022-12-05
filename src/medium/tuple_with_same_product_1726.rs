// https://leetcode.com/problems/tuple-with-same-product/

use crate::Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                *map.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }

        map.values().fold(0, |acc, &n| acc + n * (n - 1) * 4)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 3, 4, 6];
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 4, 5, 10];
        let result = Solution::tuple_same_product(nums);
        assert_eq!(result, 16);
    }
}
