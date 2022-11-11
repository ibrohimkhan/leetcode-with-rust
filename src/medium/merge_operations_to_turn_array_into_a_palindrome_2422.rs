// https://leetcode.com/problems/merge-operations-to-turn-array-into-a-palindrome/

use crate::Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut nums = std::collections::VecDeque::from_iter(nums);
        let mut count = 0;

        while nums.len() >= 2 {
            let first = *nums.front().unwrap();
            let last = *nums.back().unwrap();

            if first == last {
                nums.pop_front();
                nums.pop_back();

            } else if first > last {
                let sum = nums.pop_back().unwrap() + nums.pop_back().unwrap();
                nums.push_back(sum);
                count += 1;

            } else if first < last {
                let sum = nums.pop_front().unwrap() + nums.pop_front().unwrap();
                nums.insert(0, sum);
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 3, 2, 1, 2, 3, 1];
        let result = Solution::minimum_operations(nums);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3, 4];
        let result = Solution::minimum_operations(nums);
        assert_eq!(result, 3);
    }
}
