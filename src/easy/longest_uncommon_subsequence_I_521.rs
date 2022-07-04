// https://leetcode.com/problems/longest-uncommon-subsequence-i/

use crate::Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b { -1 } else { a.len().max(b.len()) as _ }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let a = "aba".to_string();
        let b = "cdc".to_string();
        let result = Solution::find_lu_slength(a, b);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let a = "aaa".to_string();
        let b = "ccc".to_string();
        let result = Solution::find_lu_slength(a, b);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3() {
        let a = "aaa".to_string();
        let b = "aaa".to_string();
        let result = Solution::find_lu_slength(a, b);
        assert_eq!(result, -1);
    }
}