// https://leetcode.com/problems/subsets/

use crate::Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];

        for num in nums {
            let mut subset = Vec::new();

            for item in &result {
                let mut item = item.clone();
                item.push(num);
                subset.push(item);
            }

            result.append(&mut subset);
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
        let result = Solution::subsets(nums);
        assert_eq!(result, vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
    }

    #[test]
    fn test_2() {
        let nums = vec![0];
        let result = Solution::subsets(nums);
        assert_eq!(result, vec![vec![], vec![0]]);
    }

    #[test]
    fn test_3() {
        let nums = vec![1, 2, 3, 4];
        let mut result = Solution::subsets(nums);

        assert_eq!(result.sort(), vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2],
            vec![2, 3],
            vec![2, 4],
            vec![2, 3, 4],
            vec![3],
            vec![3, 4],
            vec![1, 2, 3, 4],
        ].sort());
    }
}
