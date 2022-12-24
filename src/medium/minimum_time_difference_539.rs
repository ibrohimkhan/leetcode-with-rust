// https://leetcode.com/problems/minimum-time-difference/

use crate::Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        fn to_minutes(time: &String) -> i32 {
            let h = time[0..2].to_string().parse::<i32>().unwrap();
            let m = time[3..].to_string().parse::<i32>().unwrap();
            h * 60 + m
        }

        let mut minutes = time_points
            .iter()
            .map(|x| to_minutes(x))
            .collect::<Vec<_>>();

        minutes.sort();

        let mut min = minutes.first().unwrap() - minutes.last().unwrap() + 24 * 60;
        for i in 1..minutes.len() {
            min = min.min(minutes[i] - minutes[i - 1]);
        }

        min
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let time_points = vec!["23:59".to_string(), "00:00".to_string()];
        let result = Solution::find_min_difference(time_points);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let time_points = vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string(),
        ];
        let result = Solution::find_min_difference(time_points);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_3() {
        let time_points = vec![
            "05:31".to_string(),
            "22:08".to_string(),
            "00:35".to_string(),
        ];
        let result = Solution::find_min_difference(time_points);
        assert_eq!(result, 147);
    }

    #[test]
    fn test_4() {
        let time_points = vec!["12:12".to_string(), "00:13".to_string()];
        let result = Solution::find_min_difference(time_points);
        assert_eq!(result, 719);
    }
}
