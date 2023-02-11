// https://leetcode.com/problems/check-distances-between-same-letters/

use crate::Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        let v = s.chars().collect::<Vec<_>>();

        for i in 0..s.len() {
            let letter = v[i];

            if !map.contains_key(&letter) {
                map.insert(letter, i + 1);
                
            } else if distance[letter as usize - 97] != (i - map[&letter]) as i32 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "abaccb";
        let distance = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert!(Solution::check_distances(s.into(), distance));
    }

    #[test]
    fn test_2() {
        let s = "aa";
        let distance = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(Solution::check_distances(s.into(), distance), false);
    }
}
