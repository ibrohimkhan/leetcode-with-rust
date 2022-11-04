// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/

use crate::Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if (nums[i] - nums[j]).abs() == k { count += 1; }
            }
        }

        count
    }

    pub fn count_k_difference_2(nums: Vec<i32>, k: i32) -> i32 {
        let (mut map, mut count) = (std::collections::HashMap::<i32, i32>::new(), 0);
        for num in nums {
            if map.contains_key(&(num - k)) { count += map[&(num - k)]; }
            if map.contains_key(&(num + k)) { count += map[&(num + k)]; }

            *map.entry(num).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 2, 1];
        let k = 1;
        let result = Solution::count_k_difference_2(nums, k);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3];
        let k = 3;
        let result = Solution::count_k_difference_2(nums, k);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let nums = vec![3, 2, 1, 5, 4];
        let k = 2;
        let result = Solution::count_k_difference_2(nums, k);
        assert_eq!(result, 3);
    }
}
