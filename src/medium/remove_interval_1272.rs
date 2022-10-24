// https://leetcode.com/problems/remove-interval/

use crate::Solution;

impl Solution {
    pub fn remove_interval(intervals: Vec<Vec<i32>>, to_be_removed: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        
        for interval in &intervals {
            if interval[0] < to_be_removed[0] {
                result.push(vec![interval[0], interval[1].min(to_be_removed[0])]);
            }
            
            if interval[1] > to_be_removed[1] {
                result.push(vec![interval[0].max(to_be_removed[1]), interval[1]]);
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
        let intervals = vec![vec![0, 2], vec![3, 4], vec![5, 7]];
        let to_be_removed = vec![1, 6];
        let result = Solution::remove_interval(intervals, to_be_removed);
        assert_eq!(result, vec![vec![0, 1], vec![6, 7]])
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![0, 5]];
        let to_be_removed = vec![2, 3];
        let result = Solution::remove_interval(intervals, to_be_removed);
        assert_eq!(result, vec![vec![0, 2], vec![3, 5]])
    }

    #[test]
    fn test_3() {
        let intervals = vec![
            vec![-5, -4],
            vec![-3, -2],
            vec![1, 2],
            vec![3, 5],
            vec![8, 9],
        ];

        let to_be_removed = vec![-1, 4];
        let result = Solution::remove_interval(intervals, to_be_removed);
        assert_eq!(
            result,
            vec![vec![-5, -4], vec![-3, -2], vec![4, 5], vec![8, 9]]
        )
    }
}
