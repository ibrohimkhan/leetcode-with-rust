// https://leetcode.com/problems/buildings-with-an-ocean-view/

use crate::Solution;

impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for (i, v) in heights.iter().enumerate() {
            while !result.is_empty() && heights[*result.last().unwrap() as usize] <= *v {
                result.pop();
            }

            result.push(i as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let heights = vec![4, 2, 3, 1];
        let result = Solution::find_buildings(heights);
        assert_eq!(result, vec![0, 2, 3]);
    }

    #[test]
    fn test_2() {
        let heights = vec![4, 3, 2, 1];
        let result = Solution::find_buildings(heights);
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let heights = vec![1, 3, 2, 4];
        let result = Solution::find_buildings(heights);
        assert_eq!(result, vec![3]);
    }
}
