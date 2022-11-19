// https://leetcode.com/problems/design-hit-counter/

struct HitCounter {
    hits: std::collections::VecDeque<i32>
}

impl HitCounter {

    fn new() -> Self {
        Self { hits: std::collections::VecDeque::new() }
    }
    
    fn hit(&mut self, timestamp: i32) {
        self.hits.push_back(timestamp);
    }
    
    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while !self.hits.is_empty() {
            if timestamp - self.hits.front().unwrap() >= 300 { self.hits.pop_front(); }
            else { break; }
        }

        self.hits.len() as _
    }
}

#[cfg(test)]
mod tests {
    use super::HitCounter;

    #[test]
    fn test() {
        let mut hit_counter = HitCounter::new();
        hit_counter.hit(1);
        hit_counter.hit(2);
        hit_counter.hit(3);

        assert_eq!(hit_counter.get_hits(4), 3);

        hit_counter.hit(300);

        assert_eq!(hit_counter.get_hits(300), 4);
        assert_eq!(hit_counter.get_hits(301), 3);
    }
}
