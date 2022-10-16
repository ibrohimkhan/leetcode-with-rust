// https://leetcode.com/problems/elements-in-array-after-removing-and-replacing-elements/

use crate::Solution;

impl Solution {
    pub fn element_in_nums(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let max_minute = queries.iter().map(|item| item[0]).max().unwrap();
        let mut minutes = vec![];

        {
            let mut queue1 = std::collections::VecDeque::from_iter(nums.iter().map(|item| *item));
            let mut queue2 = std::collections::VecDeque::new();

            let mut flag = true;
            minutes.push(queue1.to_owned());

            for _ in 1..=max_minute {
                if flag { queue2.push_back(queue1.pop_front().unwrap()); } 
                else { queue1.push_back(queue2.pop_front().unwrap()); }

                if queue1.len() == 0 || queue1.len() == nums.len() { flag = !flag; }
                minutes.push(queue1.to_owned());
            }
        }

        let mut result = vec![];
        for query in queries {
            let (time, index) = (query[0] as usize, query[1] as usize);

            if minutes[time].is_empty() || minutes[time].len() <= index { result.push(-1); } 
            else { result.push(minutes[time][index]); }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 2];
        let queries = vec![vec![0, 2], vec![2, 0], vec![3, 2], vec![5, 0]];
        let result = Solution::element_in_nums(nums, queries);
        assert_eq!(result, vec![2, 2, -1, 0]);
    }

    #[test]
    fn test_2() {
        let nums = vec![2];
        let queries = vec![vec![0, 0], vec![1, 0], vec![2, 0], vec![3, 0]];
        let result = Solution::element_in_nums(nums, queries);
        assert_eq!(result, vec![2, -1, 2, -1]);
    }

    #[test]
    fn test_3() {
        let nums = vec![0, 1, 2];
        let queries = vec![vec![2, 2], vec![2, 2], vec![2, 2], vec![2, 2], vec![2, 2]];
        let result = Solution::element_in_nums(nums, queries);
        assert_eq!(result, vec![-1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_4() {
        let nums = vec![1, 1, 2];
        let queries = vec![vec![0, 0], vec![3, 0], vec![2, 1]];
        let result = Solution::element_in_nums(nums, queries);
        assert_ne!(result, vec![2, -1, 2, -1]);
    }
}
