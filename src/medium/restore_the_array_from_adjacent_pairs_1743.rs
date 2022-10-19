// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/

use crate::Solution;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for pair in &adjacent_pairs {
            map.entry(pair[1]).or_insert(vec![]).push(pair[0]);
            map.entry(pair[0]).or_insert(vec![]).push(pair[1]);
        }

        let start = map
            .iter()
            .find_map(|(k, v)| if v.len() == 1 { Some(*k) } else { None })
            .unwrap();

        let mut stack = vec![start];
        let mut seen = std::collections::HashSet::new();

        let mut result = vec![];
        while !stack.is_empty() {
            let current = stack.pop().unwrap();

            result.push(current);
            seen.insert(current);

            for k in map.get(&current).unwrap() {
                if !seen.contains(k) { stack.push(*k); }
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
        let adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
        let mut result = Solution::restore_array(adjacent_pairs);
        result.sort();

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
        let mut result = Solution::restore_array(adjacent_pairs);
        result.sort();

        let mut expected = vec![-2, 4, 1, -3];
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let adjacent_pairs = vec![vec![100000, -100000]];
        let mut result = Solution::restore_array(adjacent_pairs);
        result.sort();

        let mut expected = vec![100000, -100000];
        expected.sort();

        assert_eq!(result, expected);
    }
}
