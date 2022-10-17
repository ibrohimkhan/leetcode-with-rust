// https://leetcode.com/problems/peak-index-in-a-mountain-array/

use crate::Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, arr.len() - 1);
        
        while left < right {
            let mid = left + ((right - left) / 2);

            if arr[mid - 1] < arr[mid] && arr[mid] > arr[mid + 1] {
                return mid as _;
            }

            if arr[mid - 1] < arr[mid] && arr[mid] < arr[mid + 1] {
                left = mid;
            }

            if arr[mid - 1] > arr[mid] && arr[mid] > arr[mid + 1] {
                right = mid;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![0, 1, 0];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let arr = vec![0, 2, 1, 0];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let arr = vec![0, 10, 5, 2];
        let result = Solution::peak_index_in_mountain_array(arr);
        assert_eq!(result, 1);
    }
}
