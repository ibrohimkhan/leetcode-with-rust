// https://leetcode.com/problems/validate-stack-sequences/

use crate::Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut i = 0;

        for item in pushed {
            stack.push(item);

            while !stack.is_empty() && stack.last() == popped.get(i) {
                stack.pop();
                i += 1;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        let result = Solution::validate_stack_sequences(pushed, popped);
        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        let result = Solution::validate_stack_sequences(pushed, popped);
        assert_eq!(result, false);
    }
}
