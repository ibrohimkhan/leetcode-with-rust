// https://leetcode.com/problems/unique-number-of-occurrences/

use crate::Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut freqs = std::collections::HashMap::new();
        for item in arr {
            *freqs.entry(item).or_insert(0) += 1;
        }

        let uniques = freqs
            .values()
            .into_iter()
            .map(|&item| item)
            .collect::<std::collections::HashSet<i32>>();

        uniques.len() == freqs.values().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let result = Solution::unique_occurrences(arr);
        assert!(result);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2];
        let result = Solution::unique_occurrences(arr);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let result = Solution::unique_occurrences(arr);
        assert!(result);
    }
}
