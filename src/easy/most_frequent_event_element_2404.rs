// https://leetcode.com/problems/most-frequent-even-element/

use crate::Solution;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &item in nums.iter().filter(|&x| x % 2 == 0) {
            *map.entry(item).or_insert(0) += 1;
        }

        if let Some(result) = map
            .iter()
            .min_by(|&a, &b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)))
        {
            *result.0
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 2, 2, 4, 4, 1];
        assert_eq!(Solution::most_frequent_even(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![4, 4, 4, 9, 2, 4];
        assert_eq!(Solution::most_frequent_even(nums), 4);
    }

    #[test]
    fn test_3() {
        let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];
        assert_eq!(Solution::most_frequent_even(nums), -1);
    }
}
