// https://leetcode.com/problems/corporate-flight-bookings/

use crate::Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut arr = vec![vec![0; n as usize]; bookings.len()];
        
        for (i, book) in bookings.iter().enumerate() {
            let start = (book[0] - 1) as usize;
            let end = (book[1] - 1) as usize;
            
            for j in start..=end {
                arr[i][j] = book[2];
            }
        }

        let mut result = vec![];
        for col in 0..n as usize {
            let mut sum = 0;

            for row in 0..arr.len() {
                sum += arr[row][col];
            }

            result.push(sum);
        }

        result
    }

    pub fn corp_flight_bookings_2(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut result = vec![0; n as usize + 1];
        
        for book in bookings {
            result[book[0] as usize - 1] += book[2];
            result[book[1] as usize] -= book[2];
        }

        for i in 1..n as usize {
            result[i] += result[i - 1];
        }

        result[0..n as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        let n = 5;
        let result = Solution::corp_flight_bookings_2(bookings, n);
        assert_eq!(result, vec![10, 55, 45, 25, 25]);
    }

    #[test]
    fn test_2() {
        let bookings = vec![vec![1, 2, 10], vec![2, 2, 15]];
        let n = 2;
        let result = Solution::corp_flight_bookings_2(bookings, n);
        assert_eq!(result, vec![10, 25]);
    }
}
