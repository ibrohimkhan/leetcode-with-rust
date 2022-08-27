// https://leetcode.com/problems/container-with-most-water/

use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        
        while left < right {
            let current_area = height[left].min(height[right]) * (right - left) as i32;
            max = max.max(current_area);

            if height[left] < height[right] { left += 1; } else { right -= 1; }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let result = Solution::max_area(height);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_2() {
        let height = vec![1,1];
        let result = Solution::max_area(height);
        assert_eq!(result, 1);
    }
}