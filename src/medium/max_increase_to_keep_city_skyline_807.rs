// https://leetcode.com/problems/max-increase-to-keep-city-skyline/

// problem video explanation: https://www.youtube.com/watch?v=F3KTBw4PGUs

use crate::Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let len = grid.len();

        let mut max_rows = vec![0; len];
        let mut max_cols = vec![0; len];

        for i in 0..len {
            max_rows[i] = *grid[i].iter().max().unwrap();

            for j in 0..len {
                max_cols[j] = max_cols[j].max(grid[i][j]);
            }
        }

        let mut result = 0;
        for i in 0..len {
            for j in 0..len {
                result += max_rows[i].min(max_cols[j]) - grid[i][j];
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
        let grid = vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ];

        let result = Solution::max_increase_keeping_skyline(grid);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];

        let result = Solution::max_increase_keeping_skyline(grid);
        assert_eq!(result, 0);
    }
}
