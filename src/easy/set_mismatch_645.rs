// https://leetcode.com/problems/set-mismatch/

use crate::Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for &item in nums.iter() {
            *map.entry(item).or_insert(0) += 1;
        }

        let (mut duplicate, mut missing) = (-1, 1);

        for i in 1..=nums.len() as i32 {
            if map.contains_key(&i) {
                if let Some(&val) = map.get(&i) {
                    if val == 2 { duplicate = i; }
                }
            } else {
                missing = i;
            }
        }

        vec![duplicate, missing]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 2, 4];
        assert_eq!(Solution::find_error_nums(nums), vec![2, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_error_nums(nums), vec![1, 2]);
    }
}
