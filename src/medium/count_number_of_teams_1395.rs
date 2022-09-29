// https://leetcode.com/problems/count-number-of-teams/

use crate::Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut count = 0;

        for i in 0..rating.len() - 2 {
            for j in i..rating.len() - 1 {
                for k in j..rating.len() {
                    if rating[i] < rating[j] && rating[j] < rating[k] { count += 1; }
                    if rating[i] > rating[j] && rating[j] > rating[k] { count += 1; }
                }
            }
        }

        count
    }

    pub fn num_teams_dp(rating: Vec<i32>) -> i32 {
        let mut count = 0;
        
        for i in 1..rating.len() {
            let (mut right_less, mut right_more, mut left_less, mut left_more) = (0, 0, 0, 0);

            for j in 0..i {
                if rating[j] > rating[i] { left_more += 1; }
                else if rating[j] < rating[i] { left_less += 1; }
            }

            for k in i + 1..rating.len() {
                if rating[k] > rating[i] { right_more += 1; }
                else if rating[k] < rating[i] { right_less += 1; }
            }

            count += right_more * left_less + left_more * right_less;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let rating = vec![2, 5, 3, 4, 1];
        let result = Solution::num_teams_dp(rating);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let rating = vec![2, 1, 3];
        let result = Solution::num_teams_dp(rating);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let rating = vec![1, 2, 3, 4];
        let result = Solution::num_teams(rating);
        assert_eq!(result, 4);
    }
}
