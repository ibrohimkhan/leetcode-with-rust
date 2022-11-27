// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs-ii/

use crate::Solution;

impl Solution {
    pub fn minimum_time(mut jobs: Vec<i32>, mut workers: Vec<i32>) -> i32 {
        jobs.sort_by(|a, b| b.cmp(a));
        workers.sort_by(|a, b| b.cmp(a));

        let (mut result, len) = (0, jobs.len());
        for i in 0..len {
            let mut day = jobs[i] / workers[i];
            if jobs[i] % workers[i] != 0 {
                day += 1;
            }

            result = result.max(day);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let jobs = vec![5, 2, 4];
        let workers = vec![1, 7, 5];
        let result = Solution::minimum_time(jobs, workers);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let jobs = vec![3, 18, 15, 9];
        let workers = vec![6, 5, 1, 3];
        let result = Solution::minimum_time(jobs, workers);
        assert_eq!(result, 3);
    }
}
