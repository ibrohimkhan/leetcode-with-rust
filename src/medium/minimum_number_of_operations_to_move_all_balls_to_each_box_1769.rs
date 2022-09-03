// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/

use crate::Solution;

impl Solution {
    pub fn min_operations_1769(boxes: String) -> Vec<i32> {
        let boxes = boxes.chars().collect::<Vec<_>>();
        
        let mut result = vec![0; boxes.len()];
        let (mut ops, mut count) = (0, 0);

        for i in 0..boxes.len() {
            result[i] += ops;
            count += if boxes[i] == '1' { 1 } else { 0 };
            ops += count;
        }

        ops = 0; count = 0;
        for i in (0..boxes.len()).rev() {
            result[i] += ops;
            count += if boxes[i] == '1' { 1 } else { 0 };
            ops += count;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let boxes = String::from("110");
        let result = Solution::min_operations_1769(boxes);
        assert_eq!(result, vec![1, 1, 3]);
    }

    #[test]
    fn test_2() {
        let boxes = String::from("001011");
        let result = Solution::min_operations_1769(boxes);
        assert_eq!(result, vec![11, 8, 5, 4, 3, 4]);
    }
}
