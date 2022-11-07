// https://leetcode.com/problems/sort-transformed-array/

use crate::Solution;

impl Solution {
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut result = nums
            .iter()
            .map(|num| num * num * a + num * b + c)
            .collect::<Vec<i32>>();

        result.sort();
        result
    }

    pub fn sort_transformed_array_2(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        fn quadrat(x: i32, a: i32, b: i32, c: i32) -> i32 {
            a * x * x + b * x + c
        }

        let (mut i, mut j) = (0, nums.len() - 1);
        let mut index: i32 = if a >= 0 { nums.len() as i32 - 1 } else { 0 };
        let mut result = vec![0; nums.len()];

        while i <= j {
            if a >= 0 {
                let quad_i = quadrat(nums[i], a, b, c);
                let quad_j = quadrat(nums[j], a, b, c);

                result[index as usize] = if quad_i >= quad_j { i += 1; quad_i } else { j -= 1; quad_j };
                index -= 1;

            } else {
                let quad_i = quadrat(nums[i], a, b, c);
                let quad_j = quadrat(nums[j], a, b, c);

                result[index as usize] = if quad_i >= quad_j { j -= 1; quad_j } else { i += 1; quad_i };
                index += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![-4, -2, 2, 4];
        let a = 1;
        let b = 3;
        let c = 5;
        let result = Solution::sort_transformed_array_2(nums, a, b, c);
        assert_eq!(result, vec![3, 9, 15, 33]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-4, -2, 2, 4];
        let a = -1;
        let b = 3;
        let c = 5;
        let result = Solution::sort_transformed_array(nums, a, b, c);
        assert_eq!(result, vec![-23, -5, 1, 7]);
    }
}
