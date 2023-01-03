// https://leetcode.com/problems/longest-consecutive-sequence/

use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = std::collections::HashSet::<i32>::from_iter(nums.into_iter());

        let mut result = 0;
        for num in &set {
            if !set.contains(&(*num - 1)) {
                let mut current_num = *num;
                let mut current_result = 1;

                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_result += 1;
                }

                result = current_result.max(result);
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
        let nums = vec![100, 4, 200, 1, 3, 2];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = Solution::longest_consecutive(nums);
        assert_eq!(result, 9);
    }
}
