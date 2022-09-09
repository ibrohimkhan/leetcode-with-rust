// https://leetcode.com/problems/maximum-number-of-coins-you-can-get/

use crate::Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort();
        
        let (mut left, mut right, mut sum) = (0, piles.len() - 1, 0);
        while left < right {
            sum += piles[right - 1];
            right -= 2; left += 1;
        }

        sum
    }

    // VecDeque impl
    pub fn max_coins2(mut piles: Vec<i32>) -> i32 {
        piles.sort();

        let mut deque = std::collections::VecDeque::from(piles);
        let mut sum = 0;

        while !deque.is_empty() {
            deque.pop_back();
            sum += deque.pop_back().unwrap();
            deque.pop_front();
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let piles = vec![2, 4, 1, 2, 7, 8];
        let result = Solution::max_coins(piles);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_2() {
        let piles = vec![2, 4, 5];
        let result = Solution::max_coins(piles);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3() {
        let piles = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];
        let result = Solution::max_coins(piles);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_4() {
        let piles = vec![9, 8, 7, 6, 5, 1, 2, 3, 4];
        let result = Solution::max_coins2(piles);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_5() {
        let piles = vec![2, 4, 5];
        let result = Solution::max_coins2(piles);
        assert_eq!(result, 4);
    }
}
