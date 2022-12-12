// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/

use crate::Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (grid.len(), grid[0].len());
        
        let (mut ones_row, mut zeros_row) = (vec![0; rows], vec![0; rows]);
        let (mut ones_col, mut zeros_col) = (vec![0; cols], vec![0; cols]);
        
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 0 {
                    zeros_row[i] += 1;
                    zeros_col[j] += 1;
                } else {
                    ones_row[i] += 1;
                    ones_col[j] += 1;
                }
            }
        }

        let mut result = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = ones_row[i] + ones_col[j] - zeros_row[i] - zeros_col[j];
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
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let result = Solution::ones_minus_zeros(grid);
        assert_eq!(result, vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]])
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let result = Solution::ones_minus_zeros(grid);
        assert_eq!(result, vec![vec![5, 5, 5], vec![5, 5, 5]])
    }
}
