// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/

use crate::Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == nums.len() as i32 || nums.len() == 1 {
            return nums;
        }

        let mut num_index = nums.iter().enumerate().collect::<Vec<_>>();
        num_index.sort_by_key(|(_, &x)| x);

        let mut positions = vec![false; nums.len()];
        for (i, _) in num_index.into_iter().rev().take(k as usize) {
            positions[i] = true;
        }

        let mut result = vec![];
        for (i, item) in nums.iter().enumerate() {
            if positions[i] { result.push(*item); }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 1, 3, 3];
        let k = 2;
        assert_eq!(Solution::max_subsequence(nums, k), vec![3, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, -2, 3, 4];
        let k = 3;
        assert_eq!(Solution::max_subsequence(nums, k), vec![-1, 3, 4]);
    }
}
