// https://leetcode.com/problems/two-out-of-three/

use crate::Solution;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let set1 = std::collections::HashSet::<i32>::from_iter(nums1);
        let set2 = std::collections::HashSet::<i32>::from_iter(nums2);
        let set3 = std::collections::HashSet::<i32>::from_iter(nums3);

        let mut map = std::collections::HashMap::new();
        for i in set1 { *map.entry(i).or_insert(0) += 1; }
        
        for i in set2 { *map.entry(i).or_insert(0) += 1; }

        for i in set3 { *map.entry(i).or_insert(0) += 1; }

        let mut result = vec![];
        for (k, v) in map {
            if v > 1 { result.push(k); }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums1 = vec![1, 1, 3, 2];
        let nums2 = vec![2, 3];
        let nums3 = vec![3];

        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort();
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![3, 1];
        let nums2 = vec![2, 3];
        let nums3 = vec![1, 2];

        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let nums1 = vec![1, 2, 2];
        let nums2 = vec![4, 3, 3];
        let nums3 = vec![5];

        let mut result = Solution::two_out_of_three(nums1, nums2, nums3);
        result.sort();
        assert_eq!(result, vec![]);
    }
}
