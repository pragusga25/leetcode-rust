use super::Solution;
// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // HashMap to store the count of occurrences of each character in string s
        let mut hash_map: HashMap<char, u16> = HashMap::new();

        // Iterate through each character in string s
        for c1 in s.chars() {
            // Check if the character already exists in the map
            let val = hash_map.get(&c1);
            if let Some(count) = val {
                // Increment the count if the character already exists
                hash_map.insert(c1, count + 1);
            } else {
                // Insert the character into the map with a count of 1 if it doesn't exist
                hash_map.insert(c1, 1);
            }
        }

        // Iterate through each character in string t
        for c2 in t.chars() {
            // Check if the character exists in the map
            let val = hash_map.get(&c2);
            match val {
                Some(1) => {
                    // Remove the character from the map if its count is 1
                    hash_map.remove(&c2);
                }
                Some(count) => {
                    // Decrement the count of the character if its count is greater than 1
                    hash_map.insert(c2, count - 1);
                }
                None => return false, // Return false if the character doesn't exist in string s
            };
        }

        hash_map.is_empty() // Return true if the map is empty (all characters in s are consumed)
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
