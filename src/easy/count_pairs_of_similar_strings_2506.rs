// https://leetcode.com/problems/count-pairs-of-similar-strings/

use crate::Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;

        for i in 0..words.len() {
            let set1 = std::collections::BTreeSet::from_iter(words[i].chars());
            
            for j in i + 1..words.len() {
                let set2 = std::collections::BTreeSet::from_iter(words[j].chars());
                if set1 == set2 { count += 1; }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let words: Vec<String> = vec!["aba".into(), "aabb".into(), "abcd".into(), "bac".into(), "aabc".into()];
        let result = Solution::similar_pairs(words);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let words: Vec<String> = vec!["aabb".into(), "ab".into(), "ba".into()];
        let result = Solution::similar_pairs(words);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3() {
        let words: Vec<String> = vec!["nba".into(), "cab".into(), "dba".into()];
        let result = Solution::similar_pairs(words);
        assert_eq!(result, 0);
    }
}
