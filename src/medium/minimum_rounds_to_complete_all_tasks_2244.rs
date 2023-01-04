// https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/

use crate::Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for task in tasks.iter() {
            *map.entry(*task).or_insert(0) += 1;
        }

        let mut rounds = 0;
        for &val in map.values() {
            if val == 1 { return -1; }
            else if val % 3 == 0 { rounds += val / 3; }
            else { rounds += val / 3 + 1; }
        }

        rounds
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        let result = Solution::minimum_rounds(tasks);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let tasks = vec![2, 3, 3];
        let result = Solution::minimum_rounds(tasks);
        assert_eq!(result, -1);
    }
}
