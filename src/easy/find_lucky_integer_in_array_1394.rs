// https://leetcode.com/problems/find-lucky-integer-in-an-array/

use crate::Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for key in arr {
            *map.entry(key).or_insert(0) += 1;
        }

        map.iter()
            .filter(|(k, v)| k == v)
            .map(|(k, _)| *k)
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![2, 2, 3, 4];
        assert_eq!(Solution::find_lucky(arr), 2);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(Solution::find_lucky(arr), 3);
    }

    #[test]
    fn test_3() {
        let arr = vec![2, 2, 2, 3, 3];
        assert_eq!(Solution::find_lucky(arr), -1);
    }
}
