// https://leetcode.com/problems/subdomain-visit-count/

use crate::Solution;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::<String, i32>::new();
        
        for domains in cpdomains {
            let mut iter = domains.split_whitespace();
            let value = iter.next().unwrap().to_string().parse::<i32>().unwrap();
            let key = iter.next().unwrap().to_string();
            
            *map.entry(key.to_owned()).or_insert(0) += value;
            
            let sub_domains = key.split('.').collect::<Vec<_>>();
            if sub_domains.len() == 2 {
                *map.entry(sub_domains[1].to_string()).or_insert(0) += value;
                
            } else {
                *map.entry(sub_domains[2].to_string()).or_insert(0) += value;
                *map.entry(format!("{}.{}", sub_domains[1], sub_domains[2])).or_insert(0) += value;
            }
        }
        
        let mut result = vec![];
        for (key, value) in map {
            result.push(format!("{} {}", value, key));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let cpdomains = vec!["9001 discuss.leetcode.com".to_string()];

        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort();

        let mut expected = vec![
            "9001 leetcode.com".to_string(),
            "9001 discuss.leetcode.com".to_string(),
            "9001 com".to_string(),
        ];
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let cpdomains = vec![
            "900 google.mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "1 intel.mail.com".to_string(),
            "5 wiki.org".to_string(),
        ];

        let mut result = Solution::subdomain_visits(cpdomains);
        result.sort();

        let mut expected = vec![
            "901 mail.com".to_string(),
            "50 yahoo.com".to_string(),
            "900 google.mail.com".to_string(),
            "5 wiki.org".to_string(),
            "5 org".to_string(),
            "1 intel.mail.com".to_string(),
            "951 com".to_string(),
        ];
        expected.sort();

        assert_eq!(result, expected);
    }
}
