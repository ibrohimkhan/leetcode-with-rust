// https://leetcode.com/problems/minimum-number-of-operations-to-convert-time/

use crate::Solution;

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        fn to_minutes(time: String) -> i32 {
            let mut time = time.split(":");
            let hours: i32 = time.next().unwrap().parse().unwrap();
            let minutes: i32 = time.next().unwrap().parse().unwrap();

            hours * 60 + minutes
        }

        let mut current = to_minutes(current);
        let correct = to_minutes(correct);

        let mut counter = 0;
        while current < correct {
            if current + 60 <= correct {
                current += 60;

            } else if current + 15 <= correct {
                current += 15;

            } else if current + 5 <= correct {
                current += 5;

            } else {
                current += 1;
            }

            counter += 1;
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let current = String::from("02:30");
        let correct = String::from("04:35");
        let result = Solution::convert_time(current, correct);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let current = String::from("11:00");
        let correct = String::from("11:01");
        let result = Solution::convert_time(current, correct);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let current = String::from("11:00");
        let correct = String::from("11:00");
        let result = Solution::convert_time(current, correct);
        assert_eq!(result, 0);
    }
}
