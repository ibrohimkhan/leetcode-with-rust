// https://leetcode.com/problems/split-array-into-consecutive-subsequences/

use crate::Solution;

impl Solution {
    pub fn is_possible(mut nums: Vec<i32>) -> bool {
        let mut freq = std::collections::HashMap::<i32, i32>::new();
        for num in nums.iter() {
            *freq.entry(*num).or_insert(0) += 1;
        }

        for num in nums.iter_mut() {
            let mut prev = if let Some(value) = freq.get(num) { *value } else { 0 };
            if prev == 0 { continue; }

            let mut count = 0;
            while let Some(&next) = freq.get(num) {
                if next < prev { break; }
                prev = next;

                *freq.entry(*num).or_default() -= 1;
                *num += 1; 

                count += 1; 
            }

            if count < 3 { return false; }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 3, 4, 5];
        let result = Solution::is_possible(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
        let result = Solution::is_possible(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4, 4, 5];
        let result = Solution::is_possible(nums);
        assert_eq!(result, false);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let result = Solution::is_possible(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_5() {
        let nums = vec![1, 2, 3];
        let result = Solution::is_possible(nums);
        assert_eq!(result, true);
    }

    #[test]
    fn test_6() {
        let nums = vec![1, 2, 2, 3, 3, 3];
        let result = Solution::is_possible(nums);
        assert_eq!(result, false);
    }
}
