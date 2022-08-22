// https://leetcode.com/problems/matrix-diagonal-sum

use crate::Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat.len();
        if len == 1 { return mat[0][0]; }

        let mut result = 0;
        for i in 0..len {
            result += mat[i][i];
            result += mat[i][len - i - 1];
        }

        if len % 2 == 1 {
            result -= mat[(len - 1) / 2][(len - 1) / 2]
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mat = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];

        let result = Solution::diagonal_sum(mat);
        assert_eq!(result, 25);
    }

    #[test]
    fn test_2() {
        let mat = vec![
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
            vec![1,1,1,1],
        ];

        let result = Solution::diagonal_sum(mat);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_3() {
        let mat = vec![
            vec![5],
        ];

        let result = Solution::diagonal_sum(mat);
        assert_eq!(result, 5);
    }
}