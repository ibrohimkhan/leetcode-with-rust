// https://leetcode.com/problems/missing-number/

use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32 + 1;
        let set = std::collections::HashSet::<i32>::from_iter(nums);

        for i in 0..n {
            if !set.contains(&i) {
                return i;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let result = Solution::missing_number(nums);
        assert_eq!(result, 8);
    }
}
