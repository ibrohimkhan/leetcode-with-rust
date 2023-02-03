// https://leetcode.com/problems/first-missing-positive/

use crate::Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut arr = [false; 100_001];
        arr[0] = true;

        for &item in nums.iter().filter(|&x| *x > 0 && *x < 100_001) {
            arr[item as usize] = true;
        }

        for (i, &value) in arr.iter().enumerate() {
            if value == false { return i as i32; }
        }

        100_001
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![3, 4, -1, 1];
        assert_eq!(Solution::first_missing_positive(nums), 2);
    }

    #[test]
    fn test_3() {
        let nums = vec![7, 8, 9, 11, 12];
        assert_eq!(Solution::first_missing_positive(nums), 1);
    }
}
