// https://leetcode.com/problems/positions-of-large-groups/

use crate::Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();

        let mut result = vec![];
        let mut i = 0;
        for j in 0..len {
            if j == len - 1 || s[j] != s[j + 1] {
                if j - i + 1 >= 3 {
                    result.push(vec![i as i32, j as i32]);
                }

                i = j + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = String::from("abbxxxxzzy");
        let result = Solution::large_group_positions(s);
        assert_eq!(result, vec![vec![3, 6]]);
    }

    #[test]
    fn test_2() {
        let s = String::from("abc");
        let result = Solution::large_group_positions(s);
        assert_eq!(result, Vec::<Vec<_>>::new());
    }

    #[test]
    fn test_3() {
        let s = String::from("abcdddeeeeaabbbcd");
        let result = Solution::large_group_positions(s);
        assert_eq!(result, vec![vec![3, 5], vec![6, 9], vec![12, 14]]);
    }
}