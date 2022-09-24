// https://leetcode.com/problems/integer-break/

use crate::Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 4 { n - 1 } 
        else { i32::pow(3, ((n - 2) / 3) as u32) * (n - ((n - 2) / 3) * 3) }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 2;
        let result = Solution::integer_break(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let n = 10;
        let result = Solution::integer_break(n);
        assert_eq!(result, 36);
    }
}
