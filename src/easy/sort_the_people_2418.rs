// https://leetcode.com/problems/sort-the-people/

use crate::Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for i in 0..names.len() {
            map.insert(heights[i], names[i].to_owned());
        }

        heights.sort();
        let mut sorted_names = vec![];
        
        for height in heights.iter().rev() {
            let name = map[height].to_owned();
            sorted_names.push(name);
        }

        sorted_names
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let names = vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()];
        let heights = vec![180, 165, 170];
        let result = Solution::sort_people(names, heights);
        assert_eq!(
            result,
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }

    #[test]
    fn test_2() {
        let names = vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()];
        let heights = vec![155, 185, 150];
        let result = Solution::sort_people(names, heights);
        assert_eq!(
            result,
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
