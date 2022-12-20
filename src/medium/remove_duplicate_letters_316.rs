// https://leetcode.com/problems/remove-duplicate-letters/

use crate::Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut freq = [0; 26];
        for ch in s.as_bytes() {
            freq[(ch - b'a') as usize] += 1;
        }

        let mut visited = [false; 26];
        let mut result = vec![];
        
        for &ch in s.as_bytes() {
            let index = (ch - b'a') as usize;

            freq[index] -= 1;
            if visited[index] {
                continue;
            }

            while let Some(&top) = result.last() {
                let index = (top as u8 - b'a') as usize;
                if top as u8 > ch && freq[index] > 0 {
                    visited[index] = false;
                    result.pop();

                } else {
                    break;
                }
            }

            result.push(ch as char);
            visited[index] = true;
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "bcabc".to_string();
        let result = Solution::remove_duplicate_letters(s);
        assert_eq!(result, "abc".to_string());
    }

    #[test]
    fn test_2() {
        let s = "cbacdcbc".to_string();
        let result = Solution::remove_duplicate_letters(s);
        assert_eq!(result, "acdb".to_string());
    }
}
