// https://leetcode.com/problems/strong-password-checker-ii/

use crate::Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let has_min_len = password.len() >= 8;
        let has_lowercasee_letter = password.chars().any(|ch| ('a'..='z').contains(&ch));
        let has_uppercase_letter = password.chars().any(|ch| ('A'..='Z').contains(&ch));
        let has_digit = password.chars().any(|ch| ch.is_ascii_digit());

        let special_chars = String::from("!@#$%^&*()-+");
        let has_special_char = password.chars().any(|ch| special_chars.contains([ch]));

        for i in 1..password.len() {
            let a = &password[i - 1..i];
            let b = &password[i..i + 1];
            if a.to_owned() == b.to_owned() { return false; }
        }

        has_min_len
            && has_lowercasee_letter
            && has_uppercase_letter
            && has_digit
            && has_special_char
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let result = Solution::strong_password_checker_ii(String::from("@abbCdefghi2"));
        assert_eq!(result, false);

        let result = Solution::strong_password_checker_ii(String::from("!ABCDEFGh1"));
        assert_eq!(result, true);

        let result = Solution::strong_password_checker_ii(String::from("@abCdefghi2"));
        assert_eq!(result, true);

        let result = Solution::strong_password_checker_ii(String::from("abCdefghi"));
        assert_eq!(result, false);

        let result = Solution::strong_password_checker_ii(String::from("abCdefghi3"));
        assert_eq!(result, false);
    }

    #[test]
    fn test_1() {
        let password = "IloveLe3tcode!".to_string();
        let result = Solution::strong_password_checker_ii(password);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let password = "Me+You--IsMyDream".to_string();
        let result = Solution::strong_password_checker_ii(password);
        assert_eq!(result, false);
    }

    #[test]
    fn test_3() {
        let password = "1aB!".to_string();
        let result = Solution::strong_password_checker_ii(password);
        assert_eq!(result, false);
    }
}
