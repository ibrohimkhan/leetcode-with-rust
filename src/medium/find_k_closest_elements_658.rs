// https://leetcode.com/problems/find-k-closest-elements/

use crate::Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut result = arr;

        result.sort_by(|&a, &b| i32::abs(a - x).cmp(&i32::abs(b - x)));
        result = result
            .iter()
            .take(k as usize)
            .map(|item| *item)
            .collect::<Vec<_>>();

        result.sort();
        result
    }

    pub fn find_closest_elements_binary_search(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, arr.len() - k as usize);

        while left < right {
            let mid = (left + right) / 2;

            if x - arr[mid] > arr[mid + k as usize] - x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        arr[left..left + k as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let result = Solution::find_closest_elements(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let result = Solution::find_closest_elements(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_3() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let result = Solution::find_closest_elements_binary_search(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_4() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let result = Solution::find_closest_elements_binary_search(arr, k, x);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
