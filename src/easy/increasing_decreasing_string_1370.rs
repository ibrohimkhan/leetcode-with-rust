// https://leetcode.com/problems/increasing-decreasing-string/

use crate::Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut array = [0; 26];
        for ch in s.chars() {
            array[ch as usize - 97] += 1;
        }

        let mut result = String::new();
        let mut a_to_z = true;
        let mut length = s.len();

        while length > 0 {
            for i in 0..26 {
                let j = if a_to_z { i as usize } else { 25 - i };
                if array[j] == 0 { continue; }
                
                result.push((j as u8 + 97) as char);

                array[j] -= 1;
                length -= 1;
            }

            a_to_z = !a_to_z;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "aaaabbbbcccc".to_string();
        let result = Solution::sort_string(s);
        assert_eq!(result, "abccbaabccba".to_string());
    }

    #[test]
    fn test_2() {
        let s = "rat".to_string();
        let result = Solution::sort_string(s);
        assert_eq!(result, "art".to_string());
    }
}
