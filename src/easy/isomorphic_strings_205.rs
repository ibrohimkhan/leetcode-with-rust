// https://leetcode.com/problems/isomorphic-strings/

use crate::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map1 = std::collections::HashMap::<char, usize>::new();
        let mut map2 = std::collections::HashMap::<char, usize>::new();

        for (i, (ch1, ch2)) in s.chars().zip(t.chars()).enumerate() {
            if map1.insert(ch1, i) != map2.insert(ch2, i) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("egg");
        let t = String::from("add");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let s = String::from("foo");
        let t = String::from("bar");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let s = String::from("paper");
        let t = String::from("title");
        let result = Solution::is_isomorphic(s, t);
        assert_eq!(result, true);
    }
}
