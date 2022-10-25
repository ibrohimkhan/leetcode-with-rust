// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/

use crate::Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut matrix = matrix.concat();
        matrix.sort();
        matrix[k as usize - 1]
    }

    pub fn kth_smallest_heapified(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = matrix
            .iter()
            .flat_map(|item| item.iter().map(|&val| std::cmp::Reverse(val)))
            .collect::<std::collections::BinaryHeap<_>>();

        for _ in 0..k - 1 {
            heap.pop();
        }

        heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
        let k = 8;
        let result = Solution::kth_smallest_heapified(matrix, k);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![-5]];
        let k = 1;
        let result = Solution::kth_smallest_heapified(matrix, k);
        assert_eq!(result, -5);
    }

    #[test]
    fn test_3() {
        let matrix = vec![vec![1, 2], vec![1, 3]];
        let k = 2;
        let result = Solution::kth_smallest(matrix, k);
        assert_eq!(result, 1);
    }
}
