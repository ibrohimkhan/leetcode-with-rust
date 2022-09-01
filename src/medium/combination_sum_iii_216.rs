// https://leetcode.com/problems/combination-sum-iii/

use crate::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtrack(n: i32, k: usize, begin: i32, comb: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if n == 0 && comb.len() == k {
                result.push(comb.to_vec());

            } else if n > 0 && comb.len() < k {
                for i in begin..=9 {
                    comb.push(i);
                    backtrack(n - i, k, i + 1, comb, result);
                    comb.pop();
                }
            }
        }

        let (mut result, mut comb) = (vec![], vec![]);
        backtrack(n, k as usize, 1, &mut comb, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let (k, n) = (3, 7);
        let result = Solution::combination_sum3(k, n);
        assert_eq!(result, vec![vec![1, 2, 4]]);
    }

    #[test]
    fn test_2() {
        let (k, n) = (3, 9);
        let result = Solution::combination_sum3(k, n);
        assert_eq!(result, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    }

    #[test]
    fn test_3() {
        let (k, n) = (4, 1);
        let result = Solution::combination_sum3(k, n);
        assert!(result.is_empty());
    }
}
