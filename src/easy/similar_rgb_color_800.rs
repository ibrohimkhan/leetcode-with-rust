// https://leetcode.com/problems/similar-rgb-color/

use crate::Solution;

impl Solution {
    pub fn similar_rgb(color: String) -> String {
        fn get_map(val: String) -> String {
            let map = std::collections::HashMap::<char, &str>::from_iter([
                ('0', "1"),
                ('1', "02"),
                ('2', "13"),
                ('3', "24"),
                ('4', "35"),
                ('5', "46"),
                ('6', "57"),
                ('7', "68"),
                ('8', "79"),
                ('9', "8a"),
                ('a', "9b"),
                ('b', "ac"),
                ('c', "bd"),
                ('d', "ce"),
                ('e', "df"),
                ('f', "e"),
            ]);

            let ch = val.chars().nth(0).unwrap();
            let items = map[&ch].to_string() + ch.to_string().as_str();
            
            let mut min_ch = ' ';
            let mut min_val = 256;

            for item in items.chars() {
                let hex_1 = i64::from_str_radix(item.to_string().repeat(2).as_str(), 16).unwrap();
                let hex_2 = i64::from_str_radix(val.as_str(), 16).unwrap();
                let diff = i64::abs(hex_1 - hex_2) as i32;

                if diff < min_val {
                    min_val = diff;
                    min_ch = item;
                }
            }

            min_ch.to_string().repeat(2)
        }

        let a = get_map(color[1..3].to_string());
        let b = get_map(color[3..5].to_string());
        let c = get_map(color[5..7].to_string());

        format!("#{}{}{}", a, b, c)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let color = "#09f166".to_string();
        let result = Solution::similar_rgb(color);
        assert_eq!(result, String::from("#11ee66"));
    }

    #[test]
    fn test_2() {
        let color = "#4e3fe1".to_string();
        let result = Solution::similar_rgb(color);
        assert_eq!(result, String::from("#5544dd"));
    }
}