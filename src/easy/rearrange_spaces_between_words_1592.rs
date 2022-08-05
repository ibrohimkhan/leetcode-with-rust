// https://leetcode.com/problems/rearrange-spaces-between-words/

use crate::Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let count_space = text.chars().filter(|&ch| ch == ' ').count();
        let words = text.split_whitespace().collect::<Vec<_>>();
        let div_mod = |x, y| (x / y, x % y);

        if words.len() == 1 { words[0].to_owned() + " ".repeat(count_space).as_str() }
        else {
            let (division, modulo) = div_mod(count_space, words.len() - 1);
            words.join(" ".repeat(division).as_str()) + " ".repeat(modulo).as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let text = "  this   is  a sentence ".to_string();
        let result = Solution::reorder_spaces(text);
        assert_eq!(result, String::from("this   is   a   sentence"));
    }

    #[test]
    fn test_2() {
        let text = " practice   makes   perfect".to_string();
        let result = Solution::reorder_spaces(text);
        assert_eq!(result, String::from("practice   makes   perfect "));
    }
}
