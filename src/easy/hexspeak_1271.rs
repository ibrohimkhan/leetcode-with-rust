// https://leetcode.com/problems/hexspeak/

use crate::Solution;

impl Solution {
    pub fn to_hexspeak(num: String) -> String {
        let mut hex = format!("{:X}", num.parse::<i64>().unwrap())
            .chars()
            .collect::<Vec<_>>();

        for i in 0..hex.len() {
            if !hex[i].is_ascii_digit() { continue; }

            match hex[i] {
                '0' => hex[i] = 'O',
                '1' => hex[i] = 'I',
                _ => return String::from("ERROR"),
            }
        }

        hex.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = String::from("257");
        let result = Solution::to_hexspeak(num);
        assert_eq!(result, String::from("IOI"));
    }

    #[test]
    fn test_2() {
        let num = String::from("3");
        let result = Solution::to_hexspeak(num);
        assert_eq!(result, String::from("ERROR"));
    }
}
