// https://leetcode.com/problems/maximum-ice-cream-bars/

use crate::Solution;

impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort();

        let (mut sum, mut count) = (0, 0);
        for i in costs {
            sum += i;
            if sum <= coins && i <= coins { count += 1; } 
            else { break; }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        let result = Solution::max_ice_cream(costs, coins);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;
        let result = Solution::max_ice_cream(costs, coins);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;
        let result = Solution::max_ice_cream(costs, coins);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_4() {
        let costs = vec![7, 3, 3, 6, 6, 6, 10, 5, 9, 2];
        let coins = 56;
        let result = Solution::max_ice_cream(costs, coins);
        assert_eq!(result, 9);
    }
}
