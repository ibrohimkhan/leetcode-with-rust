// https://leetcode.com/problems/different-ways-to-add-parentheses/

use crate::Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result = vec![];

        for (i, ch) in expression.char_indices() {
            if ch == '*' || ch == '+' || ch == '-' {
                let left = Solution::diff_ways_to_compute(expression[..i].to_owned());
                let right = Solution::diff_ways_to_compute(expression[i + 1..].to_owned());

                for left_item in left {
                    for right_item in &right {
                        match ch {
                            '*' => result.push(left_item * right_item),
                            '+' => result.push(left_item + right_item),
                            '-' => result.push(left_item - right_item),
                            _ => ()
                        }
                    }
                }
            }
        }

        if result.is_empty() {
            result.push(expression.parse::<i32>().unwrap());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let expression = "2-1-1".to_string();
        let mut result = Solution::diff_ways_to_compute(expression);
        result.sort();
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn test_2() {
        let expression = "2*3-4*5".to_string();
        let mut result = Solution::diff_ways_to_compute(expression);
        result.sort();
        assert_eq!(result, vec![-34, -14, -10, -10, 10]);
    }
}
