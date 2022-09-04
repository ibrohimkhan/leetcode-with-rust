// https://leetcode.com/problems/longest-common-subsequence-between-sorted-arrays/

use crate::Solution;

impl Solution {
    pub fn longest_common_subsequence(arrays: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for array in &arrays {
            for &item in array {
                *map.entry(item).or_insert(0) += 1;
            }
        }

        let mut result = vec![];
        for (k, v) in map {
            if v == arrays.len() {
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
        let arrays = vec![vec![1, 3, 4], vec![1, 4, 7, 9]];
        let result = Solution::longest_common_subsequence(arrays);
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_2() {
        let arrays = vec![
            vec![2, 3, 6, 8],
            vec![1, 2, 3, 5, 6, 7, 10],
            vec![2, 3, 4, 6, 9],
        ];
        let result = Solution::longest_common_subsequence(arrays);
        assert_eq!(result, vec![2, 3, 6]);
    }

    #[test]
    fn test_3() {
        let arrays = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8]];
        let result = Solution::longest_common_subsequence(arrays);
        assert_eq!(result, vec![]);
    }
}
