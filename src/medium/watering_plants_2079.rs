// https://leetcode.com/problems/watering-plants/

use crate::Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut watering_can = 0;
        let mut steps = 0;

        for (index, plant) in plants.into_iter().enumerate() {
            if watering_can < plant {
                steps += index * 2 + 1;
                watering_can = capacity - plant;
            } else {
                steps += 1;
                watering_can -= plant;
            }
        }

        steps as _
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let plants = vec![2, 2, 3, 3];
        let capacity = 5;
        let result = Solution::watering_plants(plants, capacity);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_2() {
        let plants = vec![1, 1, 1, 4, 2, 3];
        let capacity = 4;
        let result = Solution::watering_plants(plants, capacity);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_3() {
        let plants = vec![7, 7, 7, 7, 7, 7, 7];
        let capacity = 8;
        let result = Solution::watering_plants(plants, capacity);
        assert_eq!(result, 49);
    }
}
