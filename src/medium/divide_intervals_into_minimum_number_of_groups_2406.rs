// https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/


use crate::Solution;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;

        intervals.sort_by(|a, b| a[0].cmp(&b[0]).then_with(|| a[1].cmp(&b[1])));

        let mut heap = std::collections::BinaryHeap::<Reverse<i32>>::new();
        for interval in intervals {
            if let Some(Reverse(latest)) = heap.peek().to_owned() {
                if *latest < interval[0] { heap.pop(); }
            }

            heap.push(Reverse(interval[1]));
        }
        
        heap.len() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        let result = Solution::min_groups(intervals);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
        let result = Solution::min_groups(intervals);
        assert_eq!(result, 1);
    }
}
