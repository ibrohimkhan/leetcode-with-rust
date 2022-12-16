// https://leetcode.com/problems/rotating-the-box/

use crate::Solution;

impl Solution {
    pub fn rotate_the_box(mybox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let (rows, cols) = (mybox.len(), mybox[0].len());
        let mut result = vec![vec!['.'; rows]; cols];

        for i in 0..rows {
            let mut position = cols - 1;

            for j in (0..cols).rev() {
                match mybox[i][j] {
                    '*' => {
                        result[j][rows - 1 - i] = '*';
                        position = j - 1;
                    }
                    '#' => {
                        result[position][rows - 1 - i] = '#';
                        position = position.saturating_sub(1);
                    }
                    _ => (),
                }
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
        let mybox = vec![vec!['#', '.', '#']];
        let result = Solution::rotate_the_box(mybox);
        assert_eq!(result, vec![vec!['.'], vec!['#'], vec!['#']]);
    }

    #[test]
    fn test_2() {
        let mybox = vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']];
        let result = Solution::rotate_the_box(mybox);
        assert_eq!(
            result,
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }

    #[test]
    fn test_3() {
        let mybox = vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ];
        let result = Solution::rotate_the_box(mybox);
        assert_eq!(
            result,
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        );
    }
}
