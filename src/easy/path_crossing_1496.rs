// https://leetcode.com/problems/path-crossing/

use crate::Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visited = std::collections::HashSet::new();
        visited.insert((0, 0));

        let (mut x, mut y) = (0, 0);
        for ch in path.chars() {
            match ch {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => panic!("unsupported direction!")
            }

            if visited.contains(&(x, y)) { return true; }
            visited.insert((x, y));
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let path = "NES".to_string();
        let result = Solution::is_path_crossing(path);
        assert_eq!(result, false);
    }

    #[test]
    fn test_2() {
        let path = "NESWW".to_string();
        let result = Solution::is_path_crossing(path);
        assert_eq!(result, true);
    }
}