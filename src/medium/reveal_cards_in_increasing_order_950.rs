// https://leetcode.com/problems/reveal-cards-in-increasing-order/

use crate::Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        let n = deck.len();
        deck.sort();

        let (mut queue, mut result) = (std::collections::VecDeque::new(), vec![0; n]);
        for i in 0..n { queue.push_back(i); }

        for i in 0..n {
            if let Some(index) = queue.pop_front() {
                result[index] = deck[i];
            }

            if let Some(index) = queue.pop_front() {
                queue.push_back(index);
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
        let deck = vec![17, 13, 11, 2, 3, 5, 7];
        let result = Solution::deck_revealed_increasing(deck);
        assert_eq!(result, vec![2, 13, 3, 11, 5, 17, 7]);
    }

    #[test]
    fn test_2() {
        let deck = vec![1, 1000];
        let result = Solution::deck_revealed_increasing(deck);
        assert_eq!(result, vec![1, 1000]);
    }
}
