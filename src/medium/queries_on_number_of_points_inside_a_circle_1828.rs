// https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/

use crate::Solution;

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];

        for query in queries.iter() {
            let (x, y, r) = (query[0], query[1], query[2]);
            let mut count = 0;

            for point in points.iter() {
                let (xp, yp) = (point[0], point[1]);
                if (xp - x).pow(2) + (yp - y).pow(2) <= r.pow(2) {
                    count += 1;
                }
            }

            result.push(count);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let points = vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]];
        let queries = vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]];
        let result = Solution::count_points(points, queries);
        assert_eq!(result, vec![3, 2, 2]);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let queries = vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]];
        let result = Solution::count_points(points, queries);
        assert_eq!(result, vec![2, 3, 2, 4]);
    }
}
