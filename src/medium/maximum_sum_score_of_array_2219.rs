// https://leetcode.com/problems/maximum-sum-score-of-array/

use crate::Solution;

impl Solution {
    pub fn maximum_sum_score(nums: Vec<i32>) -> i64 {
        let mut result = i64::MIN;
        let (mut prefix, mut suffix) = (0i64, nums.iter().map(|&x| x as i64).sum::<i64>());

        for num in nums {
            prefix += num as i64;
            result = result.max(prefix.max(suffix) as i64);
            suffix -= num as i64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 3, -2, 5];
        let result = Solution::maximum_sum_score(nums);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_2() {
        let nums = vec![-3, -5];
        let result = Solution::maximum_sum_score(nums);
        assert_eq!(result, -3);
    }
}
