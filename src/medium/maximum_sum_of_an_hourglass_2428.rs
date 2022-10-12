// https://leetcode.com/problems/maximum-sum-of-an-hourglass/

use crate::Solution;

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[i].len() - 1 {
                let value = 
                      grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1]
                    + grid[i][j]
                    + grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1];

                max = max.max(value);
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let grid = vec![
            vec![6, 2, 1, 3],
            vec![4, 2, 1, 5],
            vec![9, 2, 8, 7],
            vec![4, 1, 2, 9],
        ];
        let result = Solution::max_sum(grid);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::max_sum(grid);
        assert_eq!(result, 35);
    }
}
