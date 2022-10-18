// https://leetcode.com/problems/find-players-with-zero-or-one-losses/

use crate::Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut winners = std::collections::HashMap::new();
        let mut loosers = std::collections::HashMap::new();

        for matche in matches.into_iter() {
            *winners.entry(matche[0]).or_insert(0) += 1;
            *loosers.entry(matche[1]).or_insert(0) += 1;
        }

        let mut absolute_winners = winners
            .keys()
            .filter(|&key| !loosers.contains_key(key))
            .map(|item| *item)
            .collect::<Vec<i32>>();

        let mut lost_one_time_winners = loosers
            .into_iter()
            .filter(|(_, value)| *value <= 1)
            .map(|item| item.0)
            .collect::<Vec<i32>>();

        absolute_winners.sort();
        lost_one_time_winners.sort();

        vec![absolute_winners, lost_one_time_winners]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let matches = vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1, 2, 10], vec![4, 5, 7, 8]]);
    }

    #[test]
    fn test_2() {
        let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
        let result = Solution::find_winners(matches);
        assert_eq!(result, vec![vec![1, 2, 5, 6], vec![]]);
    }
}
