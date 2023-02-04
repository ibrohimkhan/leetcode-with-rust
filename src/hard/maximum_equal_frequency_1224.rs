// https://leetcode.com/problems/maximum-equal-frequency/

use crate::Solution;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count_map = std::collections::HashMap::new();
        let mut freq_map = std::collections::HashMap::new();
        let mut result = 0;

        for (index, &value) in nums.iter().enumerate() {
            *count_map.entry(value).or_insert(0) += 1;
            
            let mut count = 0;
            if let Some(&freq) = count_map.get(&value) {
                *freq_map.entry(freq).or_insert(0) += 1;
                count = freq_map.get(&freq).unwrap() * freq;
            }

            if count == index + 1 && index != nums.len() - 1 {
                result = result.max(index + 2);
            } else if count == index {
                result = result.max(index + 1);
            }
        }

        result as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![2, 2, 1, 1, 5, 3, 3, 5];
        assert_eq!(Solution::max_equal_freq(nums), 7);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5];
        assert_eq!(Solution::max_equal_freq(nums), 13);
    }
}
