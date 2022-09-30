// https://leetcode.com/problems/battleships-in-a-board/

use crate::Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == '.' { continue; }
                if i > 0 && board[i - 1][j] == 'X' { continue; }
                if j > 0 && board[i][j - 1] == 'X' { continue; }
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
        let board = vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
        ];
        let result = Solution::count_battleships(board);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let board = vec![vec!['.']];
        let result = Solution::count_battleships(board);
        assert_eq!(result, 0);
    }
}
