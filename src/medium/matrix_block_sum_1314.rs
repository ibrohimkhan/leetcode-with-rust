// https://leetcode.com/problems/matrix-block-sum/

use crate::Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        fn sum(mat: &Vec<Vec<i32>>, k: i32, x: i32, y: i32) -> i32 {
            let mut sum = 0;

            for i in 0..mat.len() {
                for j in 0..mat[i].len() {
                    if x - k <= i as i32 && i as i32 <= x + k && y - k <= j as i32 && j as i32 <= y + k {
                        sum += mat[i][j];
                    }
                }
            }

            sum
        }

        let mut result = vec![vec![0; mat[0].len()]; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                result[i][j] = sum(&mat, k, i as i32, j as i32);
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
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let result = Solution::matrix_block_sum(mat, k);
        assert_eq!(
            result,
            vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
        );
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 2;
        let result = Solution::matrix_block_sum(mat, k);
        assert_eq!(
            result,
            vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]
        );
    }
}
