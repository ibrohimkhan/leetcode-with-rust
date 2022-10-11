// https://leetcode.com/problems/minimum-adjacent-swaps-to-make-a-valid-array/

use crate::Solution;

impl Solution {
    pub fn minimum_swaps(nums: Vec<i32>) -> i32 {
        let (mut index_max, mut index_min) = (0, -1);
        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();

        for i in 0..nums.len() {
            if nums[i] == min && index_min == -1 {
                index_min = i as i32;
            } else if nums[i] == max {
                index_max = i as i32;
            }
        }

        let result = nums.len() as i32 - 1 - index_max + index_min;
        
        if index_min > index_max { result - 1 }
        else { result }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![3, 4, 5, 5, 3, 1];
        let result = Solution::minimum_swaps(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![9];
        let result = Solution::minimum_swaps(nums);
        assert_eq!(result, 0);
    }
}
