// https://leetcode.com/problems/sum-of-even-numbers-after-queries/

use crate::Solution;

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = Vec::new();
        let mut sum = nums.iter().filter(|&x| x % 2 == 0).sum();
        
        for query in queries {
            if nums[query[1] as usize] % 2 == 0 {
                sum -= nums[query[1] as usize];
            }

            nums[query[1] as usize] += query[0];
            if nums[query[1] as usize] % 2 == 0 {
                sum += nums[query[1] as usize];
            }
            
            answer.push(sum);
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]];
        let result = Solution::sum_even_after_queries(nums, queries);
        assert_eq!(result, vec![8, 6, 2, 4]);
    }

    #[test]
    fn test_2() {
        let nums = vec![1];
        let queries = vec![vec![4, 0]];
        let result = Solution::sum_even_after_queries(nums, queries);
        assert_eq!(result, vec![0]);
    }
}
