// https://leetcode.com/problems/subrectangle-queries/

#[allow(dead_code)]
struct SubrectangleQueries {
    matrix: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { matrix: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.matrix[i as usize][j as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.matrix[row as usize][col as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::SubrectangleQueries;

    #[test]
    fn test_1() {
        let mut sub_rect_qu = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);

        let q = sub_rect_qu.get_value(0, 2);
        assert_eq!(q, 1);

        sub_rect_qu.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(
            sub_rect_qu.matrix,
            vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5],]
        );
    }
}
