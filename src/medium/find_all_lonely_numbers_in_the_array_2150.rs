// https://leetcode.com/problems/find-all-lonely-numbers-in-the-array/

use crate::Solution;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }

        map.iter()
            .filter(|(&key, &val)| {
                let (prev, next) = (key - 1, key + 1);
                val == 1 && !map.contains_key(&prev) && !map.contains_key(&next)
            })
            .map(|(&key, _)| *key)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![10, 6, 5, 8];
        let mut result = Solution::find_lonely(nums);
        result.sort();

        let mut expected = vec![10, 8];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 3, 5, 3];
        let mut result = Solution::find_lonely(nums);
        result.sort();

        let mut expected = vec![1, 5];
        expected.sort();
        assert_eq!(result, expected);
    }
}
