// https://leetcode.com/problems/koko-eating-bananas/

use crate::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut left, mut right) = (1, *piles.iter().max().unwrap());

        while left < right {
            let k = left + (right - left) / 2;
            let hours = piles.iter().fold(0i32, |acc, &p| acc + (p + k - 1) / k);

            if hours > h { left = k + 1; } 
            else { right = k; }
        }

        right
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_3() {
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;
        let result = Solution::min_eating_speed(piles, h);
        assert_eq!(result, 23);
    }
}
