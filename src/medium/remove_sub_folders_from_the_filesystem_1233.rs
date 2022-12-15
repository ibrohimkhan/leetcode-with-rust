// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/

use crate::Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort();
        
        let mut result = vec![];
        for f in folder {
            if result.is_empty() || !f.starts_with(format!("{}{}", result.last().unwrap(), "/").as_str()) {
                result.push(f);
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
        let folder: Vec<String> = vec![
            "/a".into(),
            "/a/b".into(),
            "/c/d".into(),
            "/c/d/e".into(),
            "/c/f".into(),
        ];

        let result = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a".into(), "/c/d".into(), "/c/f".into()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let folder: Vec<String> = vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into()];

        let result = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a".into()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        let folder: Vec<String> = vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()];

        let result = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()];
        assert_eq!(result, expected);
    }
}
