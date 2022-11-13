// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/

use crate::Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut prev, mut count) = (nums[0], 1);

        for i in 1..nums.len() {
            if nums[i] - prev > k {
                count += 1;
                prev = nums[i];
            }
        }

        count as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 1;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 2, 4, 5];
        let k = 0;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, 3);
    }
}
