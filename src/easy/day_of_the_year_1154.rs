// https://leetcode.com/problems/day-of-the-year/

use crate::Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        fn is_leap(year: i32) -> bool {
            (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
        }

        let array = date
            .split("-")
            .map(|item| item.parse().unwrap())
            .collect::<Vec<i32>>();
            
        let year = array[0];
        let month = array[1];
        let day = array[2];

        const MONTH_DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let mut count = 0;
        for i in 0..month - 1 {
            count += MONTH_DAYS[i as usize];
        }

        if month > 2 && is_leap(year) {
            count += 1;
        }
        count + day
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let date = String::from("2019-01-09");
        let result = Solution::day_of_year(date);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_2() {
        let date = String::from("2019-02-10");
        let result = Solution::day_of_year(date);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_3() {
        let date = String::from("2020-03-1");
        let result = Solution::day_of_year(date);
        assert_eq!(result, 61);
    }
}
