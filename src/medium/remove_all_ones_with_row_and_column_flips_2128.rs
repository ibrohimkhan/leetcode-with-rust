// https://leetcode.com/problems/remove-all-ones-with-row-and-column-flips/

use crate::Solution;

impl Solution {
    pub fn remove_ones(grid: Vec<Vec<i32>>) -> bool {
        for i in 0..grid.len() {
            let (mut same, mut opposite) = (true, true);

            for j in 0..grid[i].len() {
                same = same && grid[0][j] == grid[i][j];
                opposite = opposite && grid[0][j] != grid[i][j];
            }

            if !same && !opposite { return false; }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let grid = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        let result = Solution::remove_ones(grid);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![1, 1, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let result = Solution::remove_ones(grid);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![0]];
        let result = Solution::remove_ones(grid);
        assert_eq!(result, true);
    }
}
