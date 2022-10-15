// https://leetcode.com/problems/partition-array-according-to-given-pivot/

use crate::Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut small = vec![];
        let mut great = vec![];
        let mut equal = vec![];

        for item in nums {
            if item > pivot {
                great.push(item)
            }

            if item < pivot {
                small.push(item)
            }

            if item == pivot {
                equal.push(item);
            }
        }

        small.append(&mut equal);
        small.append(&mut great);

        small
    }

    pub fn pivot_array2(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut index = 0;
        
        for item in nums.into_iter() {
            if item > pivot {
                result.push(item);
            }

            if item < pivot {
                result.insert(index, item);
                index += 1;
            }

            if item == pivot {
                result.insert(index, item);
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
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot = 10;
        let result = Solution::pivot_array(nums, pivot);
        assert_eq!(result, vec![9, 5, 3, 10, 10, 12, 14]);
    }

    #[test]
    fn test_2() {
        let nums = vec![-3, 4, 3, 2];
        let pivot = 2;
        let result = Solution::pivot_array2(nums, pivot);
        assert_eq!(result, vec![-3, 2, 4, 3]);
    }
}
