// https://leetcode.com/problems/number-of-days-between-two-dates/

use crate::Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        fn is_leap(year: i32) -> bool {
            (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
        }

        fn f(date: String) -> i32 {
            const MONTH_DAYS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

            let array = date
                .split("-")
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i32>>();

            let year = array[0];
            let month = array[1];
            let mut days = array[2];

            for i in 1970..year {
                if is_leap(i) { days += 366; }
                else { days += 365; }
            }

            for i in 1..month {
                if i == 2 && is_leap(year) { days += 1; }
                days += MONTH_DAYS[i as usize - 1];
            }

            days
        }

        i32::abs(f(date1) - f(date2))
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let date1 = "2019-06-29".to_string();
        let date2 = "2019-06-30".to_string();
        let result = Solution::days_between_dates(date1, date2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_2() {
        let date1 = "2020-01-15".to_string();
        let date2 = "2019-12-31".to_string();
        let result = Solution::days_between_dates(date1, date2);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_3() {
        let date1 = "2074-09-12".to_string();
        let date2 = "1983-01-08".to_string();
        let result = Solution::days_between_dates(date1, date2);
        assert_eq!(result, 33485);
    }
}
