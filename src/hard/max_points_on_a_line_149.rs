// https://leetcode.com/problems/max-points-on-a-line/

use crate::Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 1 { return 1; }

        let mut result = 2;
        for i in 0..n {
            let mut map = std::collections::HashMap::new();
            
            for j in 0..n {
                if j == i { continue; }

                let angle = f64::atan(
                    (points[j][1] - points[i][1]) as f64 / (points[j][0] - points[i][0]) as f64,
                );

                *map.entry(format!("{}", angle)).or_insert(0) += 1;
            }

            result = result.max(map.values().max().unwrap() + 1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = Solution::max_points(points);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let points = vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ];
        let result = Solution::max_points(points);
        assert_eq!(result, 4);
    }
}
