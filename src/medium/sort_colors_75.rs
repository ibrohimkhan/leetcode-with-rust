// https://leetcode.com/problems/sort-colors/

use crate::Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut j, mut n) = (0usize, 0usize, nums.len());

        while i < n {
            match nums[i] {
                0 => {
                    nums.swap(i, j);
                    i += 1; j += 1;
                }
                2 => {
                    n -= 1;
                    nums.swap(i, n);
                }
                _ => i += 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![2, 0, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
