// https://leetcode.com/problems/minimize-product-sum-of-two-arrays/

use crate::Solution;

impl Solution {
    pub fn min_product_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut nums1, mut nums2) = (nums1, nums2);
        nums1.sort();
        nums2.sort_by(|a, b| b.cmp(a));

        let mut result = 0;
        for (a, b) in nums1.iter().zip(nums2) {
            result += a * b;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums1 = vec![5,3,4,2];
        let nums2 = vec![4,2,2,5];
        let result = Solution::min_product_sum(nums1, nums2);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![2,1,4,5,7];
        let nums2 = vec![3,2,4,8,6];
        let result = Solution::min_product_sum(nums1, nums2);
        assert_eq!(result, 65);
    }
}
