// https://leetcode.com/problems/maximum-number-of-groups-entering-a-competition/

use crate::Solution;

impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        let (mut i, mut len) = (0, grades.len());
        while i + 1 <= len {
            i += 1;
            len -= i;
        }

        i as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let grades = vec![10, 6, 12, 7, 3, 5];
        let result = Solution::maximum_groups(grades);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let grades = vec![8, 8];
        let result = Solution::maximum_groups(grades);
        assert_eq!(result, 1);
    }
}
