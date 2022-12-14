// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

use crate::Solution;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][j] > 0 {
                    matrix[i][j] *= matrix[i - 1][j - 1].min(matrix[i - 1][j].min(matrix[i][j - 1])) + 1;
                }
            }
        }

        matrix.concat().iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        let result = Solution::count_squares(matrix);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let result = Solution::count_squares(matrix);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_3() {
        let matrix = vec![
            vec![0, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
            vec![0, 1, 1, 1],
        ];
        let result = Solution::count_squares(matrix);
        assert_eq!(result, 21);
    }
}
