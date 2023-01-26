// https://leetcode.com/problems/number-of-unequal-triplets-in-array/

use crate::Solution;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;
        
        for i in 0..len {
            let first = nums[i];
            
            for j in i + 1..len {
                let second = nums[j];
                if first == second { continue; }
                
                for k in j + 1..len {
                    let third = nums[k];
                    
                    if third == first || third == second { continue; }
                    count += 1;
                }
            }
        }
        
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let nums = vec![4, 4, 2, 4, 3];
        let result = Solution::unequal_triplets(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = Solution::unequal_triplets(nums);
        assert_eq!(result, 0);
    }
}
