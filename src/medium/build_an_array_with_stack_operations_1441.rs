// https://leetcode.com/problems/build-an-array-with-stack-operations/

use crate::Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut index = 0;

        for i in 1..=n {
            let target_item = target[index];
            result.push("Push".to_string());
            
            if target_item != i {
                result.push("Pop".to_string());
                continue;
            }
            
            index += 1;
            if index == target.len() { break; }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let target = vec![1, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        assert_eq!(
            result,
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }

    #[test]
    fn test_2() {
        let target = vec![1, 2, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        assert_eq!(
            result,
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Push".to_string()
            ]
        );
    }

    #[test]
    fn test_3() {
        let target = vec![1, 2];
        let n = 4;
        let result = Solution::build_array(target, n);
        assert_eq!(
            result,
            vec![
                "Push".to_string(),
                "Push".to_string(),
            ]
        );
    }

    #[test]
    fn test_4() {
        let target = vec![2, 3, 4];
        let n = 4;
        let result = Solution::build_array(target, n);
        assert_eq!(
            result,
            vec![
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string(),
                "Push".to_string(),
                "Push".to_string(),
            ]
        );
    }
}
