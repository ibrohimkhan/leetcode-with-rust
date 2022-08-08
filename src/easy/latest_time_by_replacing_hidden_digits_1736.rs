// https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/

use crate::Solution;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.chars().collect::<Vec<_>>();

        if time[0] == '?' {
            time[0] = if time[1] <= '3' || time[1] == '?' { '2' } else { '1' };
        }

        if time[1] == '?' {
            time[1] = if time[0] == '2' { '3' } else { '9' };
        }

        if time[3] == '?' { time[3] = '5'; }
        if time[4] == '?' { time[4] = '9'; }

        time.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let time = String::from("2?:?0");
        let result = Solution::maximum_time(time);
        assert_eq!(result, String::from("23:50"));
    }

    #[test]
    fn test_2() {
        let time = String::from("0?:3?");
        let result = Solution::maximum_time(time);
        assert_eq!(result, String::from("09:39"));
    }

    #[test]
    fn test_3() {
        let time = String::from("1?:33");
        let result = Solution::maximum_time(time);
        assert_eq!(result, String::from("19:33"));
    }
}