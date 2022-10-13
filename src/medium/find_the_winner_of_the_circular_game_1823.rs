// https://leetcode.com/problems/find-the-winner-of-the-circular-game/

use crate::Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = (1..=n).collect::<std::collections::VecDeque<i32>>();

        while friends.len() > 1 {
            friends.rotate_left((k as usize - 1) % friends.len());
            friends.pop_front();
        }

        friends.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 5;
        let k = 2;
        let result = Solution::find_the_winner(n, k);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let n = 6;
        let k = 5;
        let result = Solution::find_the_winner(n, k);
        assert_eq!(result, 1);
    }
}
