// https://leetcode.com/problems/slowest-key/

use crate::Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys: Vec<char> = keys_pressed.chars().collect();
        let mut key = keys[0];
        let mut duration = release_times[0];

        for i in 1..release_times.len() {
            let current_key = keys[i];
            let current_duration = release_times[i] - release_times[i - 1];

            if current_duration > duration {
                key = current_key;
                duration = current_duration;
                
            } else if current_duration == duration && current_key > key {
                key = current_key;
            }
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let release_times = vec![9,29,49,50];
        let keys_pressed = "cbcd".to_string();
        let result = Solution::slowest_key(release_times, keys_pressed);
        assert_eq!(result, 'c');
    }

    #[test]
    fn test_2() {
        let release_times = vec![12,23,36,46,62];
        let keys_pressed = "spuda".to_string();
        let result = Solution::slowest_key(release_times, keys_pressed);
        assert_eq!(result, 'a');
    }
}