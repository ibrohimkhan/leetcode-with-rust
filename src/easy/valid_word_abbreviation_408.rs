// https://leetcode.com/problems/valid-word-abbreviation/

use crate::Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let (mut number, mut position) = (0 as usize, 0 as usize);
        for ch in abbr.chars() {
            if ch.is_ascii_digit() {
                if ch == '0' && number == 0 { return false; }
                number = ch.to_digit(10).unwrap() as usize + number * 10;
                
            } else {
                position += number;
                number = 0;
                if position >= word.len() || word.chars().nth(position).unwrap() != ch {
                    return false;
                } 

                position += 1;
            }
        }

        position + number == word.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let word = "internationalization".to_string();
        let abbr = "i12iz4n".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let word = "apple".to_string();
        let abbr = "a2e".to_string();
        let result = Solution::valid_word_abbreviation(word, abbr);
        assert_eq!(result, false);
    }
}