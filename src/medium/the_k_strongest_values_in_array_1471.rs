// https://leetcode.com/problems/the-k-strongest-values-in-an-array/

use crate::Solution;

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort();

        let m = arr[(arr.len() - 1) / 2];
        let mut v = vec![];

        for item in arr {
            v.push((item.abs_diff(m), item));
        }

        v.sort_by(|a, b| b.cmp(a));
        v.iter().take(k as usize).map(|&item| item.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 2;
        let result = Solution::get_strongest(arr, k);
        assert_eq!(result, vec![5, 1]);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 1, 3, 5, 5];
        let k = 2;
        let result = Solution::get_strongest(arr, k);
        assert_eq!(result, vec![5, 5]);
    }

    #[test]
    fn test_3() {
        let arr = vec![6, 7, 11, 7, 6, 8];
        let k = 5;
        let result = Solution::get_strongest(arr, k);
        assert_eq!(result, vec![11, 8, 6, 6, 7]);
    }
}
