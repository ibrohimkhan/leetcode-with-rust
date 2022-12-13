// https://leetcode.com/problems/permutations/
// Video explanation from NeetCode: https://www.youtube.com/watch?v=s7AvT7cGdSo

use crate::Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        if nums.len() == 1 {
            return vec![nums.to_vec()];
        }

        for _ in 0..nums.len() {
            let n = nums.remove(0);
            let mut permutations = Solution::permute(nums.to_vec());

            for item in permutations.iter_mut() {
                item.push(n);
            }

            result.extend(permutations);
            nums.push(n);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3];
        let mut result = Solution::permute(nums);
        result.sort();

        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 1];
        let mut result = Solution::permute(nums);
        result.sort();

        assert_eq!(result, vec![vec![0, 1], vec![1, 0],]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        assert_eq!(result, vec![vec![1]]);
    }
}
