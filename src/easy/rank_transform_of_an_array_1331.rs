// https://leetcode.com/problems/rank-transform-of-an-array/

use crate::Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut v = arr.clone();
        v.sort();

        let mut map = std::collections::HashMap::new();
        let mut counter = 1;
        
        for item in v.iter() {
            if !map.contains_key(item) {
                map.insert(*item, counter);
                counter += 1;
            }
        }

        let mut result = vec![0i32; arr.len()];
        for i in 0..arr.len() {
            if let Some(value) = map.get(&arr[i]) {
                result[i] = *value;
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
        let arr = vec![40, 10, 20, 30];
        assert_eq!(Solution::array_rank_transform(arr), vec![4, 1, 2, 3]);
    }

    #[test]
    fn test_2() {
        let arr = vec![100, 100, 100];
        assert_eq!(Solution::array_rank_transform(arr), vec![1, 1, 1]);
    }

    #[test]
    fn test_3() {
        let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];
        assert_eq!(
            Solution::array_rank_transform(arr),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
