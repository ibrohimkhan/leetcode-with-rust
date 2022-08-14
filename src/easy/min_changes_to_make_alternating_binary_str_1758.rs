// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

use crate::Solution;

impl Solution {
    pub fn min_operations_1(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();

        let (mut even, mut odd) = (vec![], vec![]);
        for (i, &ch) in s.iter().enumerate() {
            if i % 2 == 0 {
                even.push(ch);
            } else {
                odd.push(ch);
            }
        }

        (even.iter().filter(|&&ch| ch == '0').count() + odd.iter().filter(|&&ch| ch == '1').count())
            .min(
                even.iter().filter(|&&ch| ch == '1').count()
                    + odd.iter().filter(|&&ch| ch == '0').count(),
            ) as _
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let s = String::from("0100");
        let result = Solution::min_operations_1(s);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let s = String::from("01");
        let result = Solution::min_operations_1(s);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let s = String::from("1111");
        let result = Solution::min_operations_1(s);
        assert_eq!(result, 2);
    }
}
