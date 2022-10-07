// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

use crate::Solution;

impl Solution {
    pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        fn rotate(mat: &mut Vec<Vec<i32>>) {
            mat.reverse();
    
            for i in 0..mat.len() {
                for j in 0..i {
                    let temp = mat[i][j];
    
                    mat[i][j] = mat[j][i];
                    mat[j][i] = temp;
                }
            }
        }

        if mat == target { return true; }
        for _ in 0..3 {
            rotate(&mut mat);
            if mat == target { return true; }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mat = vec![vec![0, 1], vec![1, 0]];
        let target = vec![vec![1, 0], vec![0, 1]];
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let mat = vec![vec![0, 1], vec![1, 1]];
        let target = vec![vec![1, 0], vec![0, 1]];
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let target = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, true);
    }

    #[test]
    fn test_4() {
        let mat = vec![vec![0, 1], vec![1, 0]];
        let target = vec![vec![0, 1], vec![1, 0]];
        let result = Solution::find_rotation(mat, target);
        assert_eq!(result, true);
    }
}
