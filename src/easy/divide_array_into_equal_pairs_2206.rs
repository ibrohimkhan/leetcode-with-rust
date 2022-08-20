// https://leetcode.com/problems/divide-array-into-equal-pairs/

use crate::Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for item in nums.iter() {
            *map.entry(item).or_insert(0) += 1;
        }

        for (_ , value) in map {
            if value % 2 != 0 { return false; }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 2, 3, 2, 2, 2];
        let result = Solution::divide_array(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::divide_array(nums);
        assert_eq!(result, false);
    }
}