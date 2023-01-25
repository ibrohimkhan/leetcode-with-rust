// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/

use crate::Solution;

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
       nums.sort();
       for item in nums.iter().filter(|&x| *x < 0) {
           if let Ok(_) = nums.binary_search(&item.abs()) {
               return item.abs();
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
        let nums = vec![-1, 2, -3, 3];
        let result = Solution::find_max_k(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 10, 6, 7, -7, 1];
        let result = Solution::find_max_k(nums);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_3() {
        let nums = vec![-10, 8, 6, 7, -2, -3];
        let result = Solution::find_max_k(nums);
        assert_eq!(result, -1);
    }
}
