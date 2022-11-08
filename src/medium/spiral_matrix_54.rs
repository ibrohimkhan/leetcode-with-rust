// https://leetcode.com/problems/spiral-matrix/

use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut result = vec![];
        let (mut up, mut down, mut left, mut right) = (0usize, rows - 1, 0usize, cols - 1);

        while result.len() < rows * cols {
            for col in left..=right {
                result.push(matrix[up][col]);
            }

            up += 1;

            for row in up..=down {
                result.push(matrix[row][right]);
            }

            if let None = right.checked_sub(1) {
                break;
            }

            right -= 1;

            for col in (left..=right).rev() {
                if up > down { continue; }
                result.push(matrix[down][col]);
            }

            if let None = down.checked_sub(1) {
                break;
            }

            down -= 1;

            for row in (up..=down).rev() {
                if left > right { continue; }
                result.push(matrix[row][left]);
            }

            left += 1;            
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}
