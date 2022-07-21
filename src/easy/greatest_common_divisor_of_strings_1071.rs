// https://leetcode.com/problems/greatest-common-divisor-of-strings/

use crate::Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        fn find_greatest(a: usize, b: usize) -> usize {
            if a == 0 { return b; }
            return find_greatest(b % a, a);
        }

        if str1.to_owned() + str2.as_ref() != str2.to_owned() + str1.as_ref() {
            return String::new();
        }

        let great_common_divisor = find_greatest(str1.len(), str2.len());
        str1[..great_common_divisor].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, String::from("ABC"));
    }

    #[test]
    fn test_2() {
        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, String::from("AB"));
    }

    #[test]
    fn test_3() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let result = Solution::gcd_of_strings(str1, str2);
        assert_eq!(result, String::from(""));
    }
}
