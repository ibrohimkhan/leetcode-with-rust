// https://leetcode.com/problems/shortest-word-distance-ii/

struct WordDistance {
    positions: std::collections::HashMap<String, Vec<usize>>,
}

impl WordDistance {

    fn new(words_dict: Vec<String>) -> Self {
        let mut positions = std::collections::HashMap::new();

        for (i, value) in words_dict.iter().enumerate() {
            positions.entry(value.to_owned()).or_insert(vec![]).push(i);
        }

        Self { positions }
    }
    
    fn shortest(&self, word1: String, word2: String) -> i32 {
        let mut result = i32::MAX;
        let (pos1, pos2) = (self.positions.get(&word1).unwrap(), self.positions.get(&word2).unwrap());
        let (mut p1, mut p2) = (0, 0);

        while p1 < pos1.len() && p2 < pos2.len() {
            let dif = i32::abs(pos1[p1]  as i32 - pos2[p2] as i32);
            result = result.min(dif);

            if pos1[p1] < pos2[p2] { p1 += 1; }
            else { p2 += 1; }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::WordDistance;

    #[test]
    fn test() {
        let word_dis = WordDistance::new(vec!["practice".to_string(), "makes".to_string(), "perfect".to_string(), "coding".to_string(), "makes".to_string()]);
        assert_eq!(word_dis.shortest("coding".to_string(), "practice".to_string()), 3);
        assert_eq!(word_dis.shortest("makes".to_string(), "coding".to_string()), 1);
    }
}
