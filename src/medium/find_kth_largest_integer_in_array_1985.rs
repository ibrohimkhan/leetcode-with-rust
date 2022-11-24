// https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/

use crate::Solution;

impl Solution {
    pub fn kth_largest_number(mut nums: Vec<String>, k: i32) -> String {
        nums.sort_by(|a, b| {
            let ord = a.len().cmp(&b.len());
            
            match ord {
                std::cmp::Ordering::Equal => a.cmp(&b),
                _ => ord,
            }
        });

        nums.reverse();
        format!("{}", nums[k as usize - 1])
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![
            "3".to_string(),
            "6".to_string(),
            "7".to_string(),
            "10".to_string(),
        ];
        let k = 4;
        let result = Solution::kth_largest_number(nums, k);
        assert_eq!(result, "3".to_string());
    }

    #[test]
    fn test_2() {
        let nums = vec![
            "2".to_string(),
            "21".to_string(),
            "12".to_string(),
            "1".to_string(),
        ];
        let k = 3;
        let result = Solution::kth_largest_number(nums, k);
        assert_eq!(result, "2".to_string());
    }

    #[test]
    fn test_3() {
        let nums = vec!["0".to_string(), "0".to_string()];
        let k = 2;
        let result = Solution::kth_largest_number(nums, k);
        assert_eq!(result, "0".to_string());
    }
}
