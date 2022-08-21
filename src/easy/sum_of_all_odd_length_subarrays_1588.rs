// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/

use crate::Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..arr.len() {
            for j in (i + 1)..(arr.len() + 1) {
                if arr[i..j].len() % 2 == 1 {
                    sum += arr[i..j].iter().sum::<i32>();
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![1, 4, 2, 5, 3];
        let result = Solution::sum_odd_length_subarrays(arr);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2];
        let result = Solution::sum_odd_length_subarrays(arr);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3() {
        let arr = vec![10, 11, 12];
        let result = Solution::sum_odd_length_subarrays(arr);
        assert_eq!(result, 66);
    }
}