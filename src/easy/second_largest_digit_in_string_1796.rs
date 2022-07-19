// https://leetcode.com/problems/second-largest-digit-in-a-string/

use crate::Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        use std::collections::HashSet;

        let set: HashSet<i32> = s
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect();

        if set.is_empty() || set.len() < 2 { -1 } else {
            let mut v = Vec::from_iter(set.iter());
            v.sort();
            *v[v.len() - 2]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("dfa12321afd");
        let result = Solution::second_highest(s);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let s = String::from("abc1111");
        let result = Solution::second_highest(s);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_3() {
        let s = String::from("ck077");
        let result = Solution::second_highest(s);
        assert_eq!(result, 0);
    }
}
