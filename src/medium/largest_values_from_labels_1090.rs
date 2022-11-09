// https://leetcode.com/problems/largest-values-from-labels/

use crate::Solution;

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        mut num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut limit_counter = std::collections::HashMap::new();
        for i in labels.iter() {
            limit_counter.insert(*i, 0);
        }

        let mut data = values
            .iter()
            .zip(labels.iter())
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<(_, _)>>();

        data.sort_by(|a, b| b.0.cmp(&a.0));

        let mut sum = 0;
        for (value, label) in data {
            let limit_counter_val = limit_counter.get_mut(&label).unwrap();

            if num_wanted > 0 && *limit_counter_val < use_limit {
                *limit_counter_val += 1;
                num_wanted -= 1;

                sum += value;
            }
        }

        sum
    }

    pub fn largest_vals_from_labels_heap(
        values: Vec<i32>,
        labels: Vec<i32>,
        mut num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut limit_counter = std::collections::HashMap::new();
        let mut data = std::collections::BinaryHeap::new();

        for i in 0..values.len() {
            limit_counter.insert(labels[i], 0);
            data.push((values[i], labels[i]));
        }

        let mut sum = 0;
        while let Some((value, label)) = data.pop() {
            let limit_counter_val = limit_counter.get_mut(&label).unwrap();

            if num_wanted > 0 && *limit_counter_val < use_limit {
                *limit_counter_val += 1;
                num_wanted -= 1;

                sum += value;
            }
        }

        sum
    }

}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 1, 2, 2, 3];
        let num_wanted = 3;
        let use_limit = 1;

        let result = Solution::largest_vals_from_labels_heap(values, labels, num_wanted, use_limit);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_2() {
        let values = vec![5, 4, 3, 2, 1];
        let labels = vec![1, 3, 3, 3, 2];
        let num_wanted = 3;
        let use_limit = 2;

        let result = Solution::largest_vals_from_labels_heap(values, labels, num_wanted, use_limit);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_3() {
        let values = vec![9, 8, 8, 7, 6];
        let labels = vec![0, 0, 0, 1, 1];
        let num_wanted = 3;
        let use_limit = 1;

        let result = Solution::largest_vals_from_labels_heap(values, labels, num_wanted, use_limit);
        assert_eq!(result, 16);
    }
}
