// https://leetcode.com/problems/equal-row-and-column-pairs/

use crate::Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let len = grid.len();

        for i in 0..len {
            'a: for j in 0..len {
                for k in 0..len {
                    if grid[i][k] != grid[k][j] {
                        continue 'a;
                    }
                }

                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        let result = Solution::equal_pairs(grid);
        assert_eq!(result, 3);
    }
}
