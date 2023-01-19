// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/

use crate::Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort();
        arr.sort();
        target == arr
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let target = vec![1, 2, 3, 4];
        let arr = vec![2, 4, 1, 3];
        let result = Solution::can_be_equal(target, arr);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let target = vec![3, 7, 9];
        let arr = vec![3, 7, 11];
        let result = Solution::can_be_equal(target, arr);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let target = vec![9];
        let arr = vec![9];
        let result = Solution::can_be_equal(target, arr);
        assert_eq!(result, true);
    }
}
