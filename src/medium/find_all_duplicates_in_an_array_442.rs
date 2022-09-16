// https://leetcode.com/problems/find-all-duplicates-in-an-array/
// video expl: https://www.youtube.com/watch?v=aMsSF1Il3IY

use crate::Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for i in 0..nums.len() {
            let index = nums[i].abs() as usize - 1;
            if nums[index] < 0 {
                result.push(index as i32 + 1);
            }

            nums[index] = -nums[index];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let result = Solution::find_duplicates(nums);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 2];
        let result = Solution::find_duplicates(nums);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let result = Solution::find_duplicates(nums);
        assert_eq!(result, vec![]);
    }
}
