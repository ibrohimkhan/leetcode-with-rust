// https://leetcode.com/problems/bulls-and-cows/

use crate::Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut s = secret
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();

        let mut g = guess
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        
        let (mut bulls, mut cows) = (0, 0);
        for i in 0..secret.len() {
            if secret.chars().nth(i) == guess.chars().nth(i) {
                s.remove(i - bulls);
                g.remove(i - bulls);
                bulls += 1;
            }
        }

        let mut map = std::collections::HashMap::new();
        for item in s {
            *map.entry(item).or_insert(0) += 1;
        }

        for item in g {
            if let Some(val) = map.get_mut(&item) {
                if *val > 0 { cows += 1; }
                *val -= 1;
            }
        }

        format!("{}A{}B", bulls, cows)
    }

    pub fn get_hint_2(secret: String, guess: String) -> String {
        let (mut bulls, mut cows) = (0, 0);

        let mut s_buff = [0; 10];
        let mut g_buff = [0; 10];

        for (s, g) in secret.as_bytes().iter().zip(guess.as_bytes()) {
            if s == g {
                bulls += 1;
            } else {
                s_buff[(s - b'0') as usize] += 1;
                g_buff[(g - b'0') as usize] += 1;
            }
        }

        for i in 0..10 {
            cows += s_buff[i].min(g_buff[i]);
        }

        format!("{}A{}B", bulls, cows)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let secret = "1807";
        let guess = "7810";
        let result = Solution::get_hint(secret.into(), guess.into());
        assert_eq!(result, "1A3B");
    }

    #[test]
    fn test_2() {
        let secret = "1123";
        let guess = "0111";
        let result = Solution::get_hint(secret.into(), guess.into());
        assert_eq!(result, "1A1B");
    }

    #[test]
    fn test_3() {
        let secret = "11";
        let guess = "11";
        let result = Solution::get_hint_2(secret.into(), guess.into());
        assert_eq!(result, "2A0B");
    }
}
