// https://leetcode.com/problems/magical-string/

use crate::Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut v = vec![1, 2, 2];
        let (mut num, mut index, mut right) = (1, 2, 3);

        while right < n {
            if let Some(&item) = v.get(index) {
                index += 1;
                v.push(num);

                if item == 1 { right += 1; } 
                else { v.push(num); right += 2; }

                num ^= 3;
            }
        }

        v.iter().take(n as usize).filter(|&&x| x == 1).count() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let n = 6;
        let result = Solution::magical_string(n);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let result = Solution::magical_string(n);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let n = 100;
        let result = Solution::magical_string(n);
        assert_eq!(result, 49);
    }
}
