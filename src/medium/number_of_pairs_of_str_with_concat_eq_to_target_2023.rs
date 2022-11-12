// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/

use crate::Solution;

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i].len() >= target.len() {continue; }

            for j in 0..nums.len() {
                if i == j || nums[j].len() >= target.len() { continue; }
                if format!("{}{}", nums[i], nums[j]) == target { count += 1; }
            }
        }

        count
    }

    pub fn num_of_pairs_map(nums: Vec<String>, target: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut count = 0;
        let t_len = target.len();

        for i in 0..nums.len() {
            let num = nums[i].to_owned();
            let n_len = num.len();

            if target.starts_with(&num) {
                let suffix = &target[n_len..];

                if map.contains_key(suffix) {
                    count += map[suffix];
                }
            }

            if target.ends_with(&num) {
                let prefix = &target[..t_len - n_len];

                if map.contains_key(prefix) {
                    count += map[prefix];
                }
            }

            *map.entry(num).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![
            "777".to_string(),
            "7".to_string(),
            "77".to_string(),
            "77".to_string(),
        ];
        let target = "7777".to_string();

        let result = Solution::num_of_pairs_map(nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![
            "123".to_string(),
            "4".to_string(),
            "12".to_string(),
            "34".to_string(),
        ];
        let target = "1234".to_string();

        let result = Solution::num_of_pairs(nums, target);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ];
        let target = "11".to_string();

        let result = Solution::num_of_pairs_map(nums, target);
        assert_eq!(result, 6);
    }
}
