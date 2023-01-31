// https://leetcode.com/problems/minimum-common-value/

use crate::Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set1 = nums1.iter().map(|x| *x).collect::<std::collections::HashSet<_>>();
        let set2 = nums2.iter().map(|x| *x).collect::<std::collections::HashSet<_>>();
        
        if let Some(&result) = set1.intersection(&set2).min() {
            result
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 3, 6];
        let nums2 = vec![2, 3, 4, 5];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5];
        let result = Solution::get_common(nums1, nums2);
        assert_eq!(result, -1);
    }
}
