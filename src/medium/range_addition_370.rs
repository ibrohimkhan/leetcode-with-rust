// https://leetcode.com/problems/range-addition/

use std::ops::Deref;

use crate::Solution;

impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; length as usize];
        for item in updates {
            for index in item[0]..=item[1] {
                result[index as usize] += item[2];
            }
        }

        result
    }

    pub fn get_modified_array_1(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; length as usize];
        for item in updates {
            let (left, right, value) = (item[0] as usize, item[1] as usize, item[2]);
            result[left] += value;

            if right < length as usize - 1 {
                result[right + 1] -= value;
            }
        }

        let mut sum = 0;
        for item in result.iter_mut() {
            sum += item.deref();
            *item = sum;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let length = 5;
        let updates = vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]];
        let result = Solution::get_modified_array_1(length, updates);
        assert_eq!(result, vec![-2, 0, 3, 5, 3]);
    }

    #[test]
    fn test_2() {
        let length = 10;
        let updates = vec![vec![2, 4, 6], vec![5, 6, 8], vec![1, 9, -4]];
        let result = Solution::get_modified_array(length, updates);
        assert_eq!(result, vec![0, -4, 2, 2, 2, 4, 4, -4, -4, -4]);
    }
}
