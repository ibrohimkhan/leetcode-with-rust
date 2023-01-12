// https://leetcode.com/problems/merge-similar-items/

use crate::Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = std::collections::HashMap::new();

        for item in items1 {
            *map.entry(item[0]).or_insert(0) += item[1];
        }

        for item in items2 {
            *map.entry(item[0]).or_insert(0) += item[1];
        }

        let mut result = vec![];
        for (k, v) in map {
            result.push(vec![k, v]);
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
        let items1 = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
        let items2 = vec![vec![3, 1], vec![1, 5]];

        let result = Solution::merge_similar_items(items1, items2);
        assert_eq!(result, vec![vec![1, 6], vec![3, 9], vec![4, 5]]);
    }

    #[test]
    fn test_2() {
        let items1 = vec![vec![1, 1], vec![3, 2], vec![2, 3]];
        let items2 = vec![vec![2, 1], vec![3, 2], vec![1, 3]];

        let result = Solution::merge_similar_items(items1, items2);
        assert_eq!(result, vec![vec![1, 4], vec![2, 4], vec![3, 4]]);
    }
}
