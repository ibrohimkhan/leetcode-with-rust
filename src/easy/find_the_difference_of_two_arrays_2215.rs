// https://leetcode.com/problems/find-the-difference-of-two-arrays/

use crate::Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1 = std::collections::HashSet::<i32>::from_iter(nums1);
        let set2 = std::collections::HashSet::<i32>::from_iter(nums2);

        let mut items1 = set1.difference(&set2).copied().collect::<Vec<_>>();
        items1.sort(); // this is for local testing

        let mut items2 = set2.difference(&set1).copied().collect::<Vec<_>>();
        items2.sort(); // this is for local testing

        vec![items1, items2]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let result = Solution::find_difference(nums1, nums2);
        assert_eq!(result, vec![vec![1, 3], vec![4, 6]]);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let result = Solution::find_difference(nums1, nums2);
        assert_eq!(result, vec![vec![3], vec![]]);
    }
}
