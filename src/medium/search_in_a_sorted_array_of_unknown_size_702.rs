// https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/

use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
