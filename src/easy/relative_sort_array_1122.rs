// https://leetcode.com/problems/relative-sort-array/

use crate::Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        arr1.sort();

        let mut result = vec![];
        for item in arr2 {
            let count = arr1.iter().filter(|&x| *x == item).count();
            let position = arr1.iter().position(|&x| x == item).unwrap();

            for _ in 0..count {
                result.push(item);
                arr1.remove(position);
            }
        }

        if !arr1.is_empty() { result.extend(arr1); }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
        let arr2 = vec![2, 1, 4, 3, 9, 6];
        let result = Solution::relative_sort_array(arr1, arr2);
        assert_eq!(result, vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19])
    }

    #[test]
    fn test_2() {
        let arr1 = vec![28, 6, 22, 8, 44, 17];
        let arr2 = vec![22, 28, 8, 6];
        let result = Solution::relative_sort_array(arr1, arr2);
        assert_eq!(result, vec![22, 28, 8, 6, 17, 44])
    }
}
