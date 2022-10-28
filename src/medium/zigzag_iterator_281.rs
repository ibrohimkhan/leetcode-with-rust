// https://leetcode.com/problems/zigzag-iterator/

#![allow(dead_code)]
struct ZigzagIterator {
    data: Vec<i32>,
    current: usize,
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut data = vec![];

        let mut count = 0usize;
        while count < v1.len() || count < v2.len() {
            if !v1.is_empty() && count < v1.len() {
                data.push(v1[count]);
            }

            if !v2.is_empty() && count < v2.len() {
                data.push(v2[count]);
            }

            count += 1;
        }

        Self { data, current: 0 }
    }

    fn next(&mut self) -> i32 {
        let item = self.data[self.current];
        self.current += 1;

        item
    }

    fn has_next(&self) -> bool {
        self.data.len() > self.current
    }
}

#[cfg(test)]
mod tests {
    use super::ZigzagIterator;

    #[test]
    fn test_1() {
        let (v1, v2) = (vec![1, 2], vec![3, 4, 5, 6]);
        let mut obj = ZigzagIterator::new(v1, v2);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.next(), 3);
        assert_eq!(obj.next(), 2);
        assert!(obj.has_next());
        assert_eq!(obj.next(), 4);
        assert_eq!(obj.next(), 5);
        assert_eq!(obj.next(), 6);
    }

    #[test]
    fn test_2() {
        let (v1, v2) = (vec![1], vec![]);
        let mut obj = ZigzagIterator::new(v1, v2);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), false);
    }

    #[test]
    fn test_3() {
        let (v1, v2) = (vec![], vec![1]);
        let mut obj = ZigzagIterator::new(v1, v2);
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), 1);
        assert_eq!(obj.has_next(), false);
    }
}
