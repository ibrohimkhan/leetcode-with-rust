// https://leetcode.com/problems/sqrtx/

use crate::Solution;

impl Solution {
    // sqrt(x) = e ** (0.5 * logx)
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 { return x; }

        let (mut left, mut right, mut mid) = (0, x, x / 2);
        loop {
            if mid <= x / mid && (mid + 1) > x / (mid + 1) { return mid; }
            else if mid > x / mid { right = mid; }
            else if mid < x / mid { left = mid; }

            mid = (left + right) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let x = 4;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let x = 8;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_3() {
        let x = 9;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 3);
    }


    #[test]
    fn test_4() {
        let x = 36;
        let result = Solution::my_sqrt(x);
        assert_eq!(result, 6);
    }
}