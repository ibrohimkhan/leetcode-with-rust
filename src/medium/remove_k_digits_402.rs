// https://leetcode.com/problems/remove-k-digits/

use crate::Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let (mut k, mut result) = (k, String::new());

        for ch in num.chars() {
            while k > 0 && result.len() > 0 && result.chars().last().unwrap() > ch {
                k -= 1;
                result.pop();
            }

            if result.is_empty() && ch == '0' { continue; }
            result.push(ch);
        }

        while !result.is_empty() && k > 0 {
            result.pop();
            k -= 1;
        }

        if !result.is_empty() { result } else { "0".to_string() }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let num = "1432219".to_string();
        let k = 3;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "1219".to_string())
    }

    #[test]
    fn test_2() {
        let num = "10200".to_string();
        let k = 1;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "200".to_string())
    }

    #[test]
    fn test_3() {
        let num = "10".to_string();
        let k = 2;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "0".to_string())
    }

    #[test]
    fn test_4() {
        let num = "9".to_string();
        let k = 1;
        let result = Solution::remove_kdigits(num, k);
        assert_eq!(result, "0".to_string())
    }
}