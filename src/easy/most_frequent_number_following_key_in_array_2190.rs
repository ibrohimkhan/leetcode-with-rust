// https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/

use crate::Solution;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map = std::collections::HashMap::new();

        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                *map.entry(nums[i + 1]).or_insert(0) += 1;
            }
        }

        map.into_iter().max_by_key(|&(_, value)| value).unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 100, 200, 1, 100];
        let key = 1;
        assert_eq!(Solution::most_frequent(nums, key), 100);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2, 2, 3];
        let key = 2;
        assert_eq!(Solution::most_frequent(nums, key), 2);
    }
}
