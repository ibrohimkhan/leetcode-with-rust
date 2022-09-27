// https://leetcode.com/problems/reduce-array-size-to-the-half/

use crate::Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for item in &arr {
            *map.entry(*item).or_insert(0) += 1;
        }

        let mut freqs =
            std::collections::BinaryHeap::from_iter(map.into_values().collect::<Vec<i32>>());

        let mut count = 0;
        let mut len = arr.len();

        while len > arr.len() / 2 {
            len -= freqs.pop().unwrap() as usize;
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let arr = vec![7, 7, 7, 7, 7, 7];
        let result = Solution::min_set_size(arr);
        assert_eq!(result, 1);
    }
}
