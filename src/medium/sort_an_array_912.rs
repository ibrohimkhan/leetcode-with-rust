// https://leetcode.com/problems/sort-an-array/

use crate::Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn partition(slice: &mut [i32]) -> usize {
            let len = slice.len();
            let pivot = slice[len - 1];
            let (mut i, mut j) = (0, 0);

            while j < len - 1 {
                if slice[j] <= pivot {
                    slice.swap(i, j);
                    i += 1;
                }

                j += 1;
            }

            slice.swap(i, len - 1);
            i
        }

        fn quick_sort(slice: &mut [i32]) {
            if !slice.is_empty() {
                let partition_index = partition(slice);

                quick_sort(&mut slice[0..partition_index]); 
                quick_sort(&mut slice[partition_index + 1..]); 
            }
        }

        quick_sort(&mut nums);
        nums
    }

    pub fn sort_array_binaryheap(nums: Vec<i32>) -> Vec<i32> {
        let max_heap = std::collections::BinaryHeap::from(nums);
        max_heap.into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![5, 2, 3, 1];
        let result = Solution::sort_array(nums);
        assert_eq!(result, vec![1, 2, 3, 5])
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let result = Solution::sort_array_binaryheap(nums);
        assert_eq!(result, vec![0, 0, 1, 1, 2, 5])
    }
}
