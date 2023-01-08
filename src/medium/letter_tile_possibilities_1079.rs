// https://leetcode.com/problems/letter-tile-possibilities/

use crate::Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        fn permute(count: &mut [i32; 26]) -> i32 {
            let mut sum = 0;
            
            for i in 0..26 {
                if count[i] > 0 {
                    count[i] -= 1;
                    sum += 1;
                    sum += permute(count);
                    count[i] += 1;
                }
            }
            
            sum
        }
        
        let mut count = [0; 26];
        for c in tiles.chars() {
            count[(c as u8 - 'A' as u8) as usize] += 1;
        }
        
        permute(&mut count)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let tiles = "AAB";
        let result = Solution::num_tile_possibilities(tiles.into());
        assert_eq!(result, 8);

        let tiles = "ABC";
        let result = Solution::num_tile_possibilities(tiles.into());
        assert_eq!(result, 15);
    }

    #[test]
    fn test_2() {
        let tiles = "AAABBC";
        let result = Solution::num_tile_possibilities(tiles.into());
        assert_eq!(result, 188);
    }

    #[test]
    fn test_3() {
        let tiles = "V";
        let result = Solution::num_tile_possibilities(tiles.into());
        assert_eq!(result, 1);
    }
}
