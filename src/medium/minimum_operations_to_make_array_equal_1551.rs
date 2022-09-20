// https://leetcode.com/problems/minimum-operations-to-make-array-equal/

use crate::Solution;

impl Solution {
    pub fn min_operations_1551(n: i32) -> i32 {
        if n % 2 == 0 { n * n / 4 } else { (n * n - 1) / 4 }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 3;
        let result = Solution::min_operations_1551(n);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let n = 6;
        let result = Solution::min_operations_1551(n);
        assert_eq!(result, 9);
    }
}
