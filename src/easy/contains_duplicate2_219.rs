// https://leetcode.com/problems/contains-duplicate-ii/

use crate::Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();

        for (i, &value) in nums.iter().enumerate() {
            if map.contains_key(&value) {
                if let Some(item) = map.get(&value) {
                    if i32::abs(i as i32 - item) <= k {
                        return true;
                    }
                }
            }
            
            map.insert(value, i as i32);
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(Solution::contains_nearby_duplicate(nums, k));
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert_eq!(Solution::contains_nearby_duplicate(nums, k), false);
    }
}
