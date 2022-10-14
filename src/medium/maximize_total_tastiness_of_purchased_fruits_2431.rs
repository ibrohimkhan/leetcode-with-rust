// https://leetcode.com/problems/maximize-total-tastiness-of-purchased-fruits/

use crate::Solution;

impl Solution {
    pub fn max_tastiness(price: Vec<i32>, tastiness: Vec<i32>, max_amount: i32, max_coupons: i32) -> i32 {
        fn find_max_tastiness(price: &Vec<i32>, tastiness: &Vec<i32>, i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if dp[i][j][k] != -1 { return dp[i][j][k] }

            if i == 0 {
                let mut cost = price[i];
                if k > 0 { cost /= 2; }

                dp[i][j][k] = 0;
                if cost <= j as i32 { dp[i][j][k] = tastiness[i]; }

                return dp[i][j][k];
            }
            
            let (c1, c2) = (price[i] as usize, price[i] as usize / 2);
            dp[i][j][k] = find_max_tastiness(price, tastiness, i - 1, j, k, dp);

            if c1 <= j {
                dp[i][j][k] = dp[i][j][k].max(tastiness[i] + find_max_tastiness(price, tastiness, i - 1, j - c1, k, dp)); 
            }
            
            if c2 <= j && k > 0 {
                dp[i][j][k] = dp[i][j][k].max(tastiness[i] + find_max_tastiness(price, tastiness, i - 1, j - c2, k - 1, dp)); 
            } 

            dp[i][j][k]
        }
        
        let (max_amount, max_coupons) = (max_amount as usize, max_coupons as usize);
        let mut dp = vec![vec![vec![-1; max_coupons + 1]; max_amount + 1]; price.len()];

        find_max_tastiness(&price, &tastiness, price.len() - 1, max_amount, max_coupons, &mut dp)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let price = vec![10, 20, 20];
        let tastiness = vec![5, 8, 8];
        let max_amount = 20;
        let max_coupons = 1;

        let result = Solution::max_tastiness(price, tastiness, max_amount, max_coupons);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_2() {
        let price = vec![10, 15, 7];
        let tastiness = vec![5, 8, 20];
        let max_amount = 10;
        let max_coupons = 2;

        let result = Solution::max_tastiness(price, tastiness, max_amount, max_coupons);
        assert_eq!(result, 28);
    }
}
