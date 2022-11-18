// https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/

use crate::Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut count = 0;
        for i in 0..=arr.len() - k as usize {
            let mut sum = 0;

            for j in i..i + k as usize {
                sum += arr[j];
            }

            if sum / k >= threshold {
                count += 1;
            }
        }

        count
    }

    pub fn num_of_subarrays_2(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let (mut count, mut sum, mut j) = (0, 0, 0);

        for i in 0..arr.len() {
            if i - j < k as usize - 1 {
                sum += arr[i];
                continue;
            }

            sum += arr[i];
            if sum / k >= threshold {
                count += 1;
            }

            sum -= arr[j];
            j += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![2, 2, 2, 2, 5, 5, 5, 8];
        let k = 3;
        let threshold = 4;
        let result = Solution::num_of_subarrays(arr, k, threshold);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let arr = vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3];
        let k = 3;
        let threshold = 5;
        let result = Solution::num_of_subarrays_2(arr, k, threshold);
        assert_eq!(result, 6);
    }
}
