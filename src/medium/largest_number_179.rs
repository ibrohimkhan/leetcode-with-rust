// https://leetcode.com/problems/largest-number/

use crate::Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str: Vec<String> = nums.iter().map(|item| item.to_string()).collect();
        nums_str.sort_by(|a, b| {
            let str1: String = format!("{}{}", b, a);
            let str2: String = format!("{}{}", a, b);
            str1.cmp(&str2)
        });

        let result = nums_str.join("");
        if result.trim_start_matches("0").is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![10, 2];
        let result = Solution::largest_number(nums);
        assert_eq!(result, "210".to_string());
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 30, 34, 5, 9];
        let result = Solution::largest_number(nums);
        assert_eq!(result, "9534330".to_string());
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 0];
        let result = Solution::largest_number(nums);
        assert_eq!(result, "0".to_string());
    }
}
