// https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/

use crate::Solution;

impl Solution {
    pub fn count_distinct_integers(mut nums: Vec<i32>) -> i32 {
        let length = nums.len() - 1;

        for i in 0..length {
            let value = nums[i]
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .ok()
                .unwrap();

            nums.push(value);
        }

        std::collections::HashSet::<i32>::from_iter(nums).len() as _
    }

    pub fn count_distinct_integers_2(nums: Vec<i32>) -> i32 {
        let mut set = nums.iter().copied().collect::<std::collections::HashSet<_>>();
        
        for mut num in nums {
            let mut rev = 0;

            while num > 0 {
                rev = (rev * 10) + (num % 10);
                num /= 10;
            }

            set.insert(rev);
        }

        set.len() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![1, 13, 10, 12, 31];
        let result = Solution::count_distinct_integers(nums);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![2, 2, 2];
        let result = Solution::count_distinct_integers_2(nums);
        assert_eq!(result, 1);
    }
}
