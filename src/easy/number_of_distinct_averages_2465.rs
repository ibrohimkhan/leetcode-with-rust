// https://leetcode.com/problems/number-of-distinct-averages/

use crate::Solution;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        nums.sort();

        while !nums.is_empty() {
            let first = *nums.first().unwrap();
            let last = *nums.last().unwrap();

            let avg = (first + last) as f32 / 2.;
            set.insert(format!("{}", avg));

            nums.remove(0);

            let index = nums.len();
            nums.remove(index - 1);
        }

        set.len() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 1, 4, 0, 3, 5];
        assert_eq!(Solution::distinct_averages(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 100];
        assert_eq!(Solution::distinct_averages(nums), 1);
    }

    #[test]
    fn test_3() {
        let nums = vec![9, 5, 7, 8, 7, 9, 8, 2, 0, 7];
        assert_eq!(Solution::distinct_averages(nums), 5);
    }
}
