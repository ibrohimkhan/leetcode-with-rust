// https://leetcode.com/problems/merge-intervals/

use crate::Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut result = vec![];
        for inter in intervals {
            let prev = match result.last_mut() {
                Some(val) => val,
                None => {
                    result.push(inter);
                    continue;
                }
            };

            if prev[1] >= inter[0] {
                *prev = vec![prev[0], prev[1].max(inter[1])];

            } else {
                result.push(inter);
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
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        let result = Solution::merge(intervals);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        let result = Solution::merge(intervals);
        assert_eq!(result, expected);
    }
}
