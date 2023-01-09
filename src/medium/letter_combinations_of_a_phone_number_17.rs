// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use crate::Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        
        let letters = std::collections::HashMap::from([
            ('2', "abc".chars().collect::<Vec<_>>()),
            ('3', "def".chars().collect::<Vec<_>>()),
            ('4', "ghi".chars().collect::<Vec<_>>()),
            ('5', "jkl".chars().collect::<Vec<_>>()),
            ('6', "mno".chars().collect::<Vec<_>>()),
            ('7', "pqrs".chars().collect::<Vec<_>>()),
            ('8', "tuv".chars().collect::<Vec<_>>()),
            ('9', "wxyz".chars().collect::<Vec<_>>()),
        ]);

        fn permute(
            digits: &Vec<char>,
            permutation: &mut String,
            idx: usize,
            res: &mut Vec<String>,
            lookup: &std::collections::HashMap<char, Vec<char>>,
        ) {
            if digits.len() == permutation.len() {
                res.push(permutation.to_owned());
                return;
            }

            let letters = lookup.get(&digits[idx]).unwrap();
            for ch in letters {
                permutation.push(*ch);
                permute(digits, permutation, idx + 1, res, lookup);
                permutation.pop();
            }
        }

        let mut result = vec![];
        permute(
            &digits.chars().collect(),
            &mut String::new(),
            0,
            &mut result,
            &letters,
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let digits = "23";
        let mut result = Solution::letter_combinations(digits.into());
        result.sort();

        assert_eq!(
            result,
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
    }

    #[test]
    fn test_2() {
        let digits = "";
        let mut result = Solution::letter_combinations(digits.into());
        result.sort();

        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_3() {
        let digits = "2";
        let mut result = Solution::letter_combinations(digits.into());
        result.sort();

        assert_eq!(
            result,
            vec!["a".to_string(), "b".to_string(), "c".to_string(),]
        );
    }
}
