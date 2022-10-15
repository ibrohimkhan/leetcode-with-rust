// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/

use crate::Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; pref.len()];
        
        arr[0] = pref[0];
        for i in 1..pref.len() {
            arr[i] = pref[i] ^ pref[i - 1];
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let pref = vec![5, 2, 0, 3, 1];
        let result = Solution::find_array(pref);
        assert_eq!(result, vec![5, 7, 2, 3, 2]);
    }

    #[test]
    fn test_2() {
        let pref = vec![13];
        let result = Solution::find_array(pref);
        assert_eq!(result, vec![13]);
    }
}
