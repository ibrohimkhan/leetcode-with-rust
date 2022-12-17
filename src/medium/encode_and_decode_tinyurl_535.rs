// https://leetcode.com/problems/encode-and-decode-tinyurl/

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

const BASE_ENCODED_URL: &str = "http://tinyurl.com/";

struct Codec {
    hash_table: HashMap<String, String>,
}

impl Codec {
    fn new() -> Self {
        Self {
            hash_table: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let hashed = {
            let mut state = DefaultHasher::new();
            long_url.hash(&mut state);
            state.finish()
        };

        let tiny_url = format!("{}{}", BASE_ENCODED_URL, hashed);
        self.hash_table.insert(tiny_url.to_owned(), long_url);
        
        tiny_url
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        self.hash_table.get(&short_url).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Codec;

    #[test]
    fn test() {
        let url = "https://leetcode.com/problems/design-tinyurl".to_string();

        let mut codec = Codec::new();
        let tiny = codec.encode(url.to_owned());
        let long = codec.decode(tiny.to_owned());

        assert_eq!(url, long);
    }
}
