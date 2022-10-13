// https://leetcode.com/problems/array-transformation/

use crate::Solution;

impl Solution {
    pub fn transform_array(mut arr: Vec<i32>) -> Vec<i32> {
        let mut changed = true;

        while changed {
            changed = false;
            
            let mut prev = arr[0];
            for i in 1..arr.len() - 1 {
                let curr = arr[i];
                let next = arr[i + 1];

                if prev < curr && curr > next {
                    changed = true;
                    arr[i] -= 1;
                }

                if prev > curr && curr < next {
                    changed = true;
                    arr[i] += 1;
                }

                prev = curr;
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let arr = vec![6, 2, 3, 4];
        let result = Solution::transform_array(arr);
        assert_eq!(result, vec![6, 3, 3, 4]);
    }

    #[test]
    fn test_2() {
        let arr = vec![1, 6, 3, 4, 3, 5];
        let result = Solution::transform_array(arr);
        assert_eq!(result, vec![1, 4, 4, 4, 4, 5]);
    }
}
