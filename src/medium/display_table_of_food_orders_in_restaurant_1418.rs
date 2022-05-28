// https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/

use std::collections::{HashMap, BTreeMap, BTreeSet};
use crate::Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut table = BTreeMap::<i32, HashMap<String, i32>>::new();
        let mut header = BTreeSet::new();

        for order in orders.iter() {
            let table_number: i32 = order[1].parse().unwrap();
            let food_name: String = order[2].to_string();

            header.insert(food_name.to_owned());
            *table.entry(table_number).or_default().entry(food_name).or_default() += 1;
        }

        let mut tabloid = Vec::<Vec<String>>::new();
        let mut titles: Vec<String> = Vec::from_iter(header.to_owned());
        titles.insert(0, "Table".to_string());
        tabloid.push(titles);

        for (&table_number, order) in table.iter() {
            let mut content = Vec::new();
            content.push(table_number.to_string());

            for item in header.iter() {
                let total_foods = *order.get(item).unwrap_or(&0);
                content.push(total_foods.to_string());
            }

            tabloid.push(content);
        }

        tabloid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let orders = vec![
            vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
            vec!["Corina".to_string(), "10".to_string(), "Beef Burrito".to_string()],
            vec!["David".to_string(), "3".to_string(), "Fried Chicken".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
            vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()],
        ];

        let result = Solution::display_table(orders);

        let expected = vec![
            vec!["Table".to_string(), "Beef Burrito".to_string(), "Ceviche".to_string(), "Fried Chicken".to_string(), "Water".to_string()],
            vec!["3".to_string(), "0".to_string(), "2".to_string(), "1".to_string(), "0".to_string()],
            vec!["5".to_string(), "0".to_string(), "1".to_string(), "0".to_string(), "1".to_string()],
            vec!["10".to_string(), "1".to_string(), "0".to_string(), "0".to_string(), "0".to_string()],
        ];

        assert_eq!(result, expected)
    }

    #[test]
    fn test_2() {
        let orders = vec![
            vec!["Laura".to_string(), "2".to_string(), "Bean Burrito".to_string()],
            vec!["Jhon".to_string(), "2".to_string(), "Beef Burrito".to_string()],
            vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()],
        ];

        let result = Solution::display_table(orders);

        let expected = vec![
            vec!["Table".to_string(), "Bean Burrito".to_string(), "Beef Burrito".to_string(), "Soda".to_string()],
            vec!["2".to_string(), "1".to_string(), "1".to_string(), "1".to_string()],
        ];

        assert_eq!(result, expected)
    }
}
