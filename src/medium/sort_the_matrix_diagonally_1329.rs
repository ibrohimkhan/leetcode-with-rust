// https://leetcode.com/problems/sort-the-matrix-diagonally/

use crate::Solution;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut map = std::collections::HashMap::<i32, std::collections::BinaryHeap<i32>>::new();

        for i in 0..m {
            for j in 00..n {
                map.entry(i as i32 - j as i32).or_default().push(-mat[i][j]);
            }
        }

        for i in 0..m {
            for j in 0..n {
                mat[i][j] = -map.entry(i as i32 - j as i32).or_default().pop().unwrap();
            }
        }

        mat
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mat = vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]];

        let result = Solution::diagonal_sort(mat);
        assert_eq!(
            result,
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3],]
        );
    }

    #[test]
    fn test_2() {
        let mat = vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50],
        ];

        let result = Solution::diagonal_sort(mat);
        assert_eq!(
            result,
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68]
            ]
        );
    }
}
