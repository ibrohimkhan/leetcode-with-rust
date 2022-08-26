// https://leetcode.com/problems/dot-product-of-two-sparse-vectors/

use std::collections::HashMap;

#[allow(dead_code)]
struct SparseVector {
    data: HashMap<usize, i32>,
}

#[allow(dead_code)]
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();
        for (key, &value) in nums.iter().enumerate() {
            if value != 0 {
                map.insert(key, value);
            }
        }

        Self { data: map }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        self.data
            .iter()
            .map(|(key, &value)| {
                if vec.data.contains_key(key) {
                    value * vec.data[key]
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::SparseVector;

    #[test]
    fn test_1() {
        let (nums1, nums2) = (vec![1, 0, 0, 2, 3], vec![0, 3, 0, 4, 0]);
        let (v1, v2) = (SparseVector::new(nums1), SparseVector::new(nums2));
        let result = v1.dot_product(v2);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2() {
        let (nums1, nums2) = (vec![0, 1, 0, 0, 0], vec![0, 0, 0, 0, 2]);
        let (v1, v2) = (SparseVector::new(nums1), SparseVector::new(nums2));
        let result = v1.dot_product(v2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let (nums1, nums2) = (vec![0, 1, 0, 0, 2, 0, 0], vec![1, 0, 0, 0, 3, 0, 4]);
        let (v1, v2) = (SparseVector::new(nums1), SparseVector::new(nums2));
        let result = v1.dot_product(v2);
        assert_eq!(result, 6);
    }
}
