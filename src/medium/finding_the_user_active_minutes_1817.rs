// https://leetcode.com/problems/finding-the-users-active-minutes/

use crate::Solution;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};

        let mut map = HashMap::<i32, HashSet<i32>>::new();
        for log in logs {
            match map.get_mut(&log[0]) {
                Some(entry) => {
                    (*entry).insert(log[1]);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(log[1]);
                    map.insert(log[0], set);
                }
            }
        }

        let mut result = vec![0; k as usize];
        for time in map.values() {
            if time.len() > 0 {
                result[time.len() - 1] += 1;
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
        let logs = vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]];
        let k = 5;
        let result = Solution::finding_users_active_minutes(logs, k);
        assert_eq!(result, vec![0, 2, 0, 0, 0]);
    }

    #[test]
    fn test_2() {
        let logs = vec![vec![1, 1], vec![2, 2], vec![2, 3]];
        let k = 4;
        let result = Solution::finding_users_active_minutes(logs, k);
        assert_eq!(result, vec![1, 1, 0, 0]);
    }
}
