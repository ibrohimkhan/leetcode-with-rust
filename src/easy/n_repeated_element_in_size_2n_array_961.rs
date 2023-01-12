// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/

use crate::Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        let n = nums.len() / 2;
        
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        
        let mut result = 0;
        for (k, v) in map {
            if v == n {
                result = k;
                break;
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 3];
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, 2);
    }
}
