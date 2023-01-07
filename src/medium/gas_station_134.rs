// https://leetcode.com/problems/gas-station/

use crate::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() { return -1; }

        let (mut start_position, mut tank) = (0, 0);

        for i in 0..cost.len() {
            tank += gas[i] - cost[i];
            if tank < 0 {
                start_position = i + 1;
                tank = 0;
            }
        }

        start_position as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let gas = vec![1, 2, 3, 4, 5];
        let cost = vec![3, 4, 5, 1, 2];
        let result = Solution::can_complete_circuit(gas, cost);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_2() {
        let gas = vec![2, 3, 4];
        let cost = vec![3, 4, 3];
        let result = Solution::can_complete_circuit(gas, cost);
        assert_eq!(result, -1);
    }
}
