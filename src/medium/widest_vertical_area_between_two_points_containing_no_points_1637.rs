// https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/

use crate::Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|item| item[0]);
        (1..points.len())
            .map(|i| (points[i][0] - points[i - 1][0]).abs())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
        let result = Solution::max_width_of_vertical_area(points);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let points = vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8],
        ];

        let result = Solution::max_width_of_vertical_area(points);
        assert_eq!(result, 3);
    }
}
