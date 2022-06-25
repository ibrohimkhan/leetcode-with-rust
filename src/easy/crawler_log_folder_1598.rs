// https://leetcode.com/problems/crawler-log-folder/

use crate::Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter().fold(0, |acc, log| {
            match log.as_str() {
                "../" if acc > 0 => acc - 1,
                "../" | "./" => acc,
                _ => acc + 1,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ];

        let result = Solution::min_operations(logs);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let logs = vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string(),
        ];

        let result = Solution::min_operations(logs);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3() {
        let logs = vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string(),
        ];

        let result = Solution::min_operations(logs);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_4() {
        let logs = vec![
            "./".to_string(),
            "../".to_string(),
            "./".to_string(),
        ];

        let result = Solution::min_operations(logs);
        assert_eq!(result, 0);
    }
}
