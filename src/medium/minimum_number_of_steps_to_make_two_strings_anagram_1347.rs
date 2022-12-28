// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/

use crate::Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freqs = std::collections::HashMap::new();
        for ch in s.chars() {
            *freqs.entry(ch).or_insert(0) += 1;
        }
        
        for ch in t.chars() {
            *freqs.entry(ch).or_insert(0) -= 1;
        }

        freqs.values().into_iter().filter(|&item| *item > 0).sum()
    }

    pub fn min_steps2(s: String, t: String) -> i32 {
        let mut arr = [0; 26];
        for i in 0..s.len() {
            arr[(s.chars().nth(i).unwrap() as u8 - b'a') as usize] += 1;
            arr[(t.chars().nth(i).unwrap() as u8 - b'a') as usize] -= 1;
        }

        arr.iter().filter(|&&x| x > 0).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "bab".to_string();
        let t = "aba".to_string();
        let result = Solution::min_steps(s, t);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = "leetcode".to_string(); // l: 1, e: 3, t: 1, c: 1, o: 1, d: 1           l o d => 3 2
        let t = "practice".to_string(); // p: 1, r: 1, a: 1, c: 2, t: 1, i: 1, e: 1
        let result = Solution::min_steps2(s, t);
        assert_eq!(result, 5);
    }
}
