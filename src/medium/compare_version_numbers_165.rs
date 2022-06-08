// https://leetcode.com/problems/compare-version-numbers/

use crate::Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        use std::cmp::Ordering;
        
        let mut nums1 = version1
            .split(".")
            .map(|item| item.parse().unwrap())
            .collect::<Vec<i32>>();

        let mut nums2 = version2
            .split(".")
            .map(|item| item.parse().unwrap())
            .collect::<Vec<i32>>();

        while let Some(&0) = nums1.last() {
            nums1.pop();
        }

        while let Some(&0) = nums2.last() {
            nums2.pop();
        }

        match nums1.cmp(&nums2) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let version1 = "1.01".to_string();
        let version2 = "1.001".to_string();
        let result = Solution::compare_version(version1, version2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_2() {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0".to_string();
        let result = Solution::compare_version(version1, version2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let version1 = "0.1".to_string();
        let version2 = "1.1".to_string();
        let result = Solution::compare_version(version1, version2);
        assert_eq!(result, -1);
    }
}
