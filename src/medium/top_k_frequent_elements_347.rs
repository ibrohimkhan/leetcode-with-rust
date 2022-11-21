// https://leetcode.com/problems/top-k-frequent-elements/

use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        let v = map.into_iter().map(|(a, b)| (b, a)).collect::<Vec<_>>();
        let mut heap = std::collections::BinaryHeap::from(v);

        let mut result = vec![];
        while let Some((_, num)) = heap.pop() {
            result.push(num);
            if result.len() == k as usize { break; }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let k = 1;
        let result = Solution::top_k_frequent(nums, k);
        assert_eq!(result, vec![1])
    }
}
