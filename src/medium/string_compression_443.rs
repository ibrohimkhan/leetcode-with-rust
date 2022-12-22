// https://leetcode.com/problems/string-compression/

use crate::Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let (mut left, mut right) = (0, 1);

        while right <= chars.len() {
            let mut count = 1;

            while right < chars.len() && chars[left] == chars[right] {
                count += 1;
                right += 1;
            }

            if count > 1 {
                chars.drain(left..right - 1);

                let mut i = left + 1;
                for ch in count.to_string().chars() {
                    chars.insert(i, ch);
                    i += 1;
                }

                right = i;
            }

            left = right;
            right += 1;
        }

        chars.len() as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_2() {
        let mut chars = vec!['a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_4() {
        let mut chars = vec!['a', 'a', 'a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_5() {
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_6() {
        let mut chars = vec![
            'p', 'p', 'p', 'p', 'm', 'm', 'b', 'b', 'b', 'b', 'b', 'u', 'u', 'r', 'r', 'u', 'n',
            'n', 'n', 'n', 'n', 'n', 'n', 'n', 'n', 'n', 'n', 'u', 'u', 'u', 'u', 'a', 'a', 'u',
            'u', 'r', 'r', 'r', 's', 's', 'a', 'a', 'y', 'y', 'y', 'g', 'g', 'g', 'g', 'g',
        ];

        let result = Solution::compress(&mut chars);
        assert_eq!(result, 30);

        let expected = vec![
            'p', '4', 'm', '2', 'b', '5', 'u', '2', 'r', '2', 'u', 'n', '1', '1', 'u', '4', 'a',
            '2', 'u', '2', 'r', '3', 's', '2', 'a', '2', 'y', '3', 'g', '5',
        ];
        assert_eq!(chars, expected);
    }

    #[test]
    fn test_7() {
        let mut chars = vec!['a', 'a', 'a', '3', '3', 'b', 'b', 'a', 'a'];
        let result = Solution::compress(&mut chars);
        assert_eq!(result, 8);

        let expected = vec![
            'a', '3', '3', '2', 'b', '2', 'a', '2'
        ];
        assert_eq!(chars, expected);
    }
}
