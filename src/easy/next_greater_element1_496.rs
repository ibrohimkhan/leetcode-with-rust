// https://leetcode.com/problems/next-greater-element-i/

use crate::Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for item in nums1 {
            let position = nums2.iter().position(|x| *x == item).unwrap();
            let element = nums2[position];

            if let Some(item) = nums2[position + 1..].into_iter().find(|&x| *x > element) {
                result.push(*item);
            } else {
                result.push(-1);
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
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let result = Solution::next_greater_element(nums1, nums2);
        assert_eq!(result, vec![-1, 3, -1]);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let result = Solution::next_greater_element(nums1, nums2);
        assert_eq!(result, vec![3, -1]);
    }
}
