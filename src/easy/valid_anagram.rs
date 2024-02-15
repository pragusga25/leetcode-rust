use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hash_map: HashMap<char, u16> = HashMap::new();

        for c1 in s.chars() {
            let val = hash_map.get(&c1);
            if val.is_some() {
                hash_map.insert(c1, val.unwrap() + 1);
            } else {
                hash_map.insert(c1, 1);
            }
        }

        for c2 in t.chars() {
            let val = hash_map.get(&c2);

            match val {
                Some(1) => {
                    hash_map.remove(&c2);
                }
                Some(val) => {
                    hash_map.insert(c2, val - 1);
                }
                None => return false,
            };
        }

        hash_map.is_empty()
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "maragan".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("taufik".to_string(), "kiftua".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("ab".to_string(), "b".to_string()),
        false
    );
    assert_eq!(Solution::is_anagram("".to_string(), "a".to_string()), false);
}
