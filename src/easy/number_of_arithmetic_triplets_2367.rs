// https://leetcode.com/problems/number-of-arithmetic-triplets/

use crate::Solution;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut result = 0;
        
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {

                if nums[j] - nums[i] == diff {
                    for k in j + 1..nums.len() {
                        if nums[k] - nums[j] == diff { result += 1; }
                    }
                }
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
        let nums = vec![0, 1, 4, 6, 7, 10];
        let diff = 3;
        let result = Solution::arithmetic_triplets(nums, diff);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 5, 6, 7, 8, 9];
        let diff = 2;
        let result = Solution::arithmetic_triplets(nums, diff);
        assert_eq!(result, 2);
    }
}
