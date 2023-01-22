// https://leetcode.com/problems/intersection-of-multiple-arrays/

use crate::Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                *map.entry(nums[i][j]).or_insert(0) += 1;
            }
        }
        
        let mut result = vec![];
        for (k, v) in map {
            if v == nums.len() {
                result.push(k);
            }
        }
        
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
        let result = Solution::intersection(nums);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_2() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let result = Solution::intersection(nums);
        assert_eq!(result, vec![]);
    }
}
