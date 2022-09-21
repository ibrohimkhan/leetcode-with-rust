// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

use crate::Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut sum = 0;
        let mut prev = 0;

        for item in bank {
            let count = item.chars().filter(|&x| x == '1').count();
            if count == 0 { continue; }

            sum += count * prev;
            prev = count;
        }

        sum as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let bank = vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string(),
        ];
        let result = Solution::number_of_beams(bank);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let bank = vec![
            "000".to_string(),
            "111".to_string(),
            "000".to_string(),
        ];
        let result = Solution::number_of_beams(bank);
        assert_eq!(result, 0);
    }
}
