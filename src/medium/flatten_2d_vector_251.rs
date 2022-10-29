// https://leetcode.com/problems/flatten-2d-vector/

#![allow(dead_code)]
struct Vector2D {
    data: Vec<i32>,
    current: usize,
}

impl Vector2D {
    fn new(v: Vec<Vec<i32>>) -> Self {
        Self {
            data: v.concat(),
            current: 0
        }
    }
    
    fn next(&mut self) -> i32 {
        let item = self.data[self.current];
        self.current += 1;
        item
    }
    
    fn has_next(&self) -> bool {
        self.current < self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Vector2D;

    #[test]
    fn test() {
        let mut v = Vector2D::new(vec![vec![1, 2], vec![3], vec![4]]);
        assert_eq!(v.next(), 1);
        assert_eq!(v.next(), 2);
        assert_eq!(v.next(), 3);
        assert_eq!(v.has_next(), true);
        assert_eq!(v.has_next(), true);
        assert_eq!(v.next(), 4);
        assert_eq!(v.has_next(), false);
    }
}
