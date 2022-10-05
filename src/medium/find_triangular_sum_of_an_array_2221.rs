// https://leetcode.com/problems/find-triangular-sum-of-an-array/

use crate::Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            let mut new_nums = vec![];

            for i in 0..nums.len() - 1 {
                new_nums.push((nums[i] + nums[i + 1]) % 10);
            }

            nums = new_nums;
        }

        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::triangular_sum(nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let nums = vec![5];
        let result = Solution::triangular_sum(nums);
        assert_eq!(result, 5);
    }
}
