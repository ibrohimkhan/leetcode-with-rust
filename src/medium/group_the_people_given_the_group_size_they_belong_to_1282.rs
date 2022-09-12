// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/

use crate::Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map = std::collections::HashMap::<i32, Vec<i32>>::new();
        let mut result = vec![];

        for i in 0..group_sizes.len() {
            let item = group_sizes[i];

            let mut item_list: Vec<i32> = if !map.contains_key(&item) {
                Vec::new()
            } else {
                map.get_mut(&item).unwrap().to_vec()
            };

            item_list.push(i as i32);
            if item_list.len() == item as usize {
                result.push(item_list.to_vec());
                map.remove(&item);
                
            } else {
                map.insert(item, item_list);
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
        let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
        let mut result = Solution::group_the_people(group_sizes);
        let mut expected = vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]];

        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let group_sizes = vec![2, 1, 3, 3, 3, 2];
        let mut result = Solution::group_the_people(group_sizes);
        let mut expected = vec![vec![1], vec![0, 5], vec![2, 3, 4]];

        result.sort();
        expected.sort();
        
        assert_eq!(result, expected);
    }
}
