// https://leetcode.com/problems/reverse-words-in-a-string-ii/

use crate::Solution;

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();

        let length = s.len();
        let (mut left, mut right) = (0, 1);
        
        while right <= length {
            if right == length || s[right] == ' ' {
                let mut p1 = left;
                let mut p2 = right - 1;

                while p1 < p2 {
                    s.swap(p1, p2);
                    p1 += 1;
                    p2 -= 1;
                }

                left = right + 1;
                right += 1;
            }

            right += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];

        Solution::reverse_words(&mut s);
        let expected = vec![
            'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
        ];

        assert_eq!(s, expected);
    }

    #[test]
    fn test_2() {
        let mut s = vec!['a'];
        Solution::reverse_words(&mut s);

        let expected = vec!['a'];
        assert_eq!(s, expected);
    }
}
