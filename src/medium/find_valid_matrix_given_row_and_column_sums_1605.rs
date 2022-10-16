// https://leetcode.com/problems/find-valid-matrix-given-row-and-column-sums/

use crate::Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                result[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= result[i][j];
                col_sum[j] -= result[i][j];
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
        let row_sum = vec![3, 8];
        let col_sum = vec![4, 7];
        let result = Solution::restore_matrix(row_sum, col_sum);
        assert_eq!(result, vec![vec![3, 0], vec![1, 7]]);
    }

    #[test]
    fn test_2() {
        let row_sum = vec![5, 7, 10];
        let col_sum = vec![8, 6, 8];
        let result = Solution::restore_matrix(row_sum, col_sum);
        assert_eq!(result, vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]]);
    }
}
