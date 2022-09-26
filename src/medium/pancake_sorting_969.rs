// https://leetcode.com/problems/pancake-sorting/

use crate::Solution;

// inspired by following solution: 
// https://leetcode.com/problems/pancake-sorting/discuss/221755/Rust-0ms-beats-100-simple-explanation.
impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut flips = Vec::new();
        let n = arr.len();

        for i in 0..n {
            let (max_position, _) = arr[0..(n - i)]
                .iter()
                .enumerate()
                .max_by(|&(_, x), &(_, y)| x.cmp(y))
                .unwrap();

            flips.push(max_position as i32 + 1);
            flips.push((n - i) as i32);

            arr[0..=max_position].reverse();
            arr[0..(n - i)].reverse();
        }

        flips
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![3, 2, 4, 1];
        let result = Solution::pancake_sort(arr);
        assert_eq!(result, vec![3, 4, 2, 3, 1, 2, 1, 1])
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 2, 3];
        let result = Solution::pancake_sort(arr);
        assert_eq!(result, vec![3, 3, 2, 2, 1, 1])
    }
}
