// https://leetcode.com/problems/insert-interval/

use crate::Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut index = 0;
        while index < intervals.len() && intervals[index][1] < new_interval[0] {
            result.push(intervals[index].to_vec());
            index += 1;
        }

        while index < intervals.len() && intervals[index][0] <= new_interval[1] {
            new_interval[0] = new_interval[0].min(intervals[index][0]);
            new_interval[1] = new_interval[1].max(intervals[index][1]);
            index += 1;
        }

        result.push(new_interval);
        while index < intervals.len() {
            result.push(intervals[index].to_vec());
            index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn test_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn test_3() {
        let intervals = vec![vec![1, 3], vec![4, 6]];
        let new_interval = vec![7, 9];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 3], vec![4, 6], vec![7, 9]]);
    }
}
