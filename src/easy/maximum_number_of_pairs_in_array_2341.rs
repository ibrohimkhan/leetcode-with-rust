// https://leetcode.com/problems/maximum-number-of-pairs-in-array/

use crate::Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for num in &nums {
            *map.entry(*num).or_insert(0) += 1;
        }

        let (mut pairs, mut leftover) = (0, 0);
        for &v in map.values() {
            pairs += v / 2;
            if v % 2 != 0 { leftover += 1; }
        }

        vec![pairs, leftover]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 3, 2, 1, 3, 2, 2];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![3, 1]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![1, 0]);
    }

    #[test]
    fn test_3() {
        let nums = vec![0];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::number_of_pairs(nums);
        assert_eq!(result, vec![0, 5]);
    }
}
