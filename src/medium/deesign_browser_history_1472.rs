// https://leetcode.com/problems/design-browser-history/

pub struct BrowserHistory {
    pages: Vec<String>,
    current: usize,
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        Self {
            pages: vec![homepage],
            current: 0,
        }
    }

    pub fn visit(&mut self, url: String) {
        self.current += 1;
        self.pages.splice(self.current.., [url]);
    }

    pub fn back(&mut self, steps: i32) -> String {
        self.current = self.current.saturating_sub(steps as usize);
        self.pages[self.current].to_owned()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        self.current = (self.current + steps as usize).min(self.pages.len() - 1);
        self.pages[self.current].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::BrowserHistory;

    #[test]
    fn test() {
        let mut browser = BrowserHistory::new(String::from("leetcode.com"));
        browser.visit(String::from("google.com"));
        browser.visit(String::from("facebook.com"));
        browser.visit(String::from("youtube.com"));

        let back = browser.back(1);
        assert_eq!(back, String::from("facebook.com"));

        let back = browser.back(2);
        assert_eq!(back, String::from("leetcode.com"));

        let forward = browser.forward(4);
        assert_eq!(forward, String::from("youtube.com"));

        let back = browser.back(10);
        assert_eq!(back, String::from("leetcode.com"));
    }
}
