// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/

use crate::Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut sum = vec![0; garbage.len()];
        for i in 1..garbage.len() {
            sum[i] = sum[i - 1] + travel[i - 1];
        }
        
        let (mut truck_g, mut truck_p, mut truck_m, mut total_time) = (0, 0, 0, 0);
        for i in 0..garbage.len() {
            total_time += garbage[i].len() as i32;

            for ch in garbage[i].chars() {
                match ch {
                    'G' => truck_g = sum[i],
                    'P' => truck_p = sum[i],
                    'M' => truck_m = sum[i],
                    _ => (),
                }
            }
        }

        total_time + truck_g + truck_p + truck_m
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let garbage = vec![
            "G".to_string(),
            "P".to_string(),
            "GP".to_string(),
            "GG".to_string(),
        ];

        let travel = vec![2, 4, 3];
        let result = Solution::garbage_collection(garbage, travel);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_2() {
        let garbage = vec![
            "MMM".to_string(),
            "PGM".to_string(),
            "GP".to_string(),
        ];

        let travel = vec![3, 10];
        let result = Solution::garbage_collection(garbage, travel);
        assert_eq!(result, 37);
    }
}
