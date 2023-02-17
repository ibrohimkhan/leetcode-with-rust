// https://leetcode.com/problems/subarray-sum-equals-k/

use crate::Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut sum = 0;

            for j in i..nums.len() {
                sum += nums[j];
                if sum == k { count += 1; }
            }
        }

        count
    }

    pub fn subarray_sum_2(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        map.insert(0, 1);
        
        let (mut count, mut sum) = (0, 0);
        
        for i in 0..nums.len() {
            sum += nums[i];

            if map.contains_key(&(sum - k)) {
                count += map[&(sum - k)];
            }

            *map.entry(sum).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        assert_eq!(Solution::subarray_sum_2(nums, k), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::subarray_sum_2(nums, k), 2);
    }
}
