// https://leetcode.com/problems/distinct-numbers-in-each-subarray/

use crate::Solution;

impl Solution {
    pub fn distinct_numbers(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::new();

        for i in 0..=(nums.len() - k as usize) {
            let mut set = std::collections::HashSet::new();

            for j in i..=(i + k as usize - 1) {
                set.insert(nums[j]);
            }

            result.push(set.len() as i32);
        }

        result
    }

    pub fn distinct_numbers1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut result = Vec::with_capacity(nums.len() - k as usize + 1);
        let (mut left, mut right) = (0, 0);

        while right < nums.len() {
            *map.entry(nums[right]).or_insert(0) += 1;

            if right - left + 1 == k as usize {
                result.push(map.len() as i32);

                let item = nums[left];
                map.insert(item, map[&item] - 1);

                if map[&item] == 0 { map.remove(&item); }
                left += 1;
            }

            right += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 2, 2, 1, 3];
        let k = 3;
        let result = Solution::distinct_numbers1(nums, k);
        assert_eq!(result, vec![3, 2, 2, 2, 3])
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1, 2, 3, 4];
        let k = 4;
        let result = Solution::distinct_numbers1(nums, k);
        assert_eq!(result, vec![1, 2, 3, 4])
    }
}
