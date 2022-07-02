// https://leetcode.com/problems/flip-game/

use crate::Solution;

impl Solution {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let current_state = current_state.chars().collect::<Vec<_>>();
        let mut result = vec![];

        for i in 0..current_state.len() - 1 {
            if current_state[i] == '+' && current_state[i + 1] == '+' {
                result.push(
                    [
                        current_state[..i].iter().collect::<String>(),
                        String::from("--"),
                        current_state[(i + 2)..].iter().collect::<String>(),
                    ]
                    .join(""),
                );
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let current_state = String::from("++++");
        let result = Solution::generate_possible_next_moves(current_state);
        assert_eq!(
            result,
            vec!["--++".to_string(), "+--+".to_string(), "++--".to_string()]
        );
    }

    #[test]
    fn test_2() {
        let current_state = String::from("+");
        let result = Solution::generate_possible_next_moves(current_state);
        assert_eq!(result, Vec::<String>::new());
    }
}
