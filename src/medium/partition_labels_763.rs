// https://leetcode.com/problems/partition-labels/

use crate::Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut arr = [0; 26];
        for (i, ch) in s.char_indices() {
            arr[ch as usize - 97] = i;
        }

        let (mut result, mut j, mut anchor) = (vec![], 0, 0);
        for (i, ch) in s.char_indices() {
            j = j.max(arr[ch as usize - 97]);
            if i == j {
                result.push(i as i32 - anchor + 1);
                anchor = i as i32 + 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let s = "ababcbacadefegdehijhklij".to_string();
        let result = Solution::partition_labels(s);
        assert_eq!(result, vec![9, 7, 8]);
    }

    #[test]
    fn test_2() {
        let s = "eccbbbbdec".to_string();
        let result = Solution::partition_labels(s);
        assert_eq!(result, vec![10]);
    }
}
