// https://leetcode.com/problems/k-closest-points-to-origin/

use crate::Solution;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_by_key(|item| item[0].pow(2) + item[1].pow(2));
        points[..k as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let result = Solution::k_closest(points, k);
        assert_eq!(result, vec![vec![-2, 2]])
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let result = Solution::k_closest(points, k);
        assert_eq!(result, vec![vec![3, 3], vec![-2, 4]])
    }
}
