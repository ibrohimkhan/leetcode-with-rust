// https://leetcode.com/problems/find-smallest-common-element-in-all-rows/

use crate::Solution;

impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::<i32, u8>::new();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                *map.entry(mat[i][j]).or_insert(0) += 1;
            }
        }

        let filtered = map
            .iter()
            .filter(|(_, &val)| val as usize >= mat.len())
            .map(|(&key, _)| key)
            .collect::<Vec<_>>();

        filtered.into_iter().min().unwrap_or(-1)
    }

    pub fn smallest_common_element_binary_search(mut mat: Vec<Vec<i32>>) -> i32 {
        for item in mat.remove(0) {
            let mut found = true;

            for row in mat.iter() {
                found = row.contains(&item);
                if !found { break; }
            }

            if found { return item; }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mat = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 4, 5, 8, 10],
            vec![3, 5, 7, 9, 11],
            vec![1, 3, 5, 7, 9],
        ];

        let result = Solution::smallest_common_element_binary_search(mat);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 2, 3], vec![2, 3, 4], vec![2, 3, 5]];

        let result = Solution::smallest_common_element_binary_search(mat);
        assert_eq!(result, 2);
    }
}
