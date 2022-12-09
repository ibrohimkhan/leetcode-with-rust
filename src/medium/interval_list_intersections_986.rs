// https://leetcode.com/problems/interval-list-intersections/

use crate::Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let (mut i, mut j, mut result) = (0, 0, vec![]);
       
        while i < first_list.len() && j < second_list.len() {
            let start = first_list[i][0].max(second_list[j][0]);
            let end = first_list[i][1].min(second_list[j][1]);

            if start <= end {
                result.push(vec![start, end]);
            }

            if first_list[i][1] < second_list[j][1] { i += 1; } 
            else { j += 1; }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let first_list = vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]];
        let second_list = vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]];
        let result = Solution::interval_intersection(first_list, second_list);
        assert_eq!(result, vec![vec![1, 2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]]);
    }

    #[test]
    fn test_2() {
        let first_list = vec![vec![1, 3], vec![5, 9]];
        let second_list = vec![];
        let result = Solution::interval_intersection(first_list, second_list);
        assert_eq!(result, Vec::<Vec<_>>::new());
    }
}
