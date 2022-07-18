// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/

use crate::Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        use std::collections::HashMap;

        if s1 == s2 { return true; }

        let mut map1 = HashMap::new();
        for ch in s1.chars() {
            *map1.entry(ch).or_insert(0) += 1;
        }

        let mut map2 = HashMap::new();
        for ch in s2.chars() {
            *map2.entry(ch).or_insert(0) += 1;
        }

        if map1 != map2 { false } else {
            let mut dif = 0;
            
            for (a, b) in s1.chars().zip(s2.chars()) {
                if a != b { dif += 1; }
            }

            dif == 2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s1 = "bank".to_string();
        let s2 = "kanb".to_string();
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s1 = "attack".to_string();
        let s2 = "defend".to_string();
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s1 = "kelb".to_string();
        let s2 = "kelb".to_string();
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, true);
    }
}
