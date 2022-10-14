// https://leetcode.com/problems/arithmetic-subarrays/

use crate::Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];

        for (i, j) in l.into_iter().zip(r) {
            let mut arr = nums[i as usize..=j as usize].to_vec();
            arr.sort();

            let mut is_eq = true;
            for i in 1..arr.len() - 1 {
                let (first, second, third) = (arr[i - 1], arr[i], arr[i + 1]);
                
                if second - first != third - second { 
                    is_eq = false;
                    break;
                }
            }

            result.push(is_eq);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 6, 5, 9, 3, 7];
        let l = vec![0, 0, 2];
        let r = vec![2, 3, 5];

        let result = Solution::check_arithmetic_subarrays(nums, l, r);
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10];
        let l = vec![0, 1, 6, 4, 8, 7];
        let r = vec![4, 4, 9, 7, 9, 10];

        let result = Solution::check_arithmetic_subarrays(nums, l, r);
        assert_eq!(result, vec![false, true, false, false, true, true]);
    }
}
