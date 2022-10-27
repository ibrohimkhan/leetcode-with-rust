// https://leetcode.com/problems/average-waiting-time/

use crate::Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut complete_time = -1;
        let mut waiting_times = vec![];

        for customer in customers {
            complete_time = if complete_time < customer[0] {
                customer[0] + customer[1]
            } else {
                complete_time + customer[1]
            };

            waiting_times.push((complete_time - customer[0]) as f64);
        }

        waiting_times.iter().sum::<f64>() / waiting_times.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
        let result = Solution::average_waiting_time(customers);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_2() {
        let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
        let result = Solution::average_waiting_time(customers);
        assert_eq!(result, 3.25);
    }
}
