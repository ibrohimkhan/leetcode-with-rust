// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/

use crate::Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let n = nums.len();
        let mut max = 0;
        for i in 0..n {
            let a = nums[i];
            let b = nums[n - 1 - i];
            max = max.max(a + b);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 5, 2, 3];
        let result = Solution::min_pair_sum(nums);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 5, 4, 2, 4, 6];
        let result = Solution::min_pair_sum(nums);
        assert_eq!(result, 8);
    }
}
