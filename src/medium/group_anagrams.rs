use super::Solution;
// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

impl Solution {
    /// Groups the anagrams in the given list of strings.
    ///
    /// Anagrams are strings that contain the same characters but in a different order.
    /// The function groups the anagrams together and returns a vector of vectors, where each inner vector
    /// contains a group of anagrams.
    ///
    /// # Arguments
    ///
    /// * `strs` - A vector of strings containing the input strings to be grouped.
    ///
    /// # Returns
    ///
    /// A vector of vectors, where each inner vector contains a group of anagrams.
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // HashMap to store sorted strings as keys and vectors of anagrams as values
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        // Iterate through each string in the input vector
        for str in strs.into_iter() {
            // Sort the string to identify anagrams
            let sorted_str = Solution::sort_str(&str);
            let val = map.get_mut(&sorted_str);

            match val {
                Some(val) => {
                    // If the sorted string already exists in the map, add the string to its corresponding vector
                    val.push(str);
                }
                None => {
                    // If the sorted string doesn't exist in the map, create a new entry with the sorted string as key
                    // and a vector containing the string as value
                    map.insert(sorted_str, vec![str]);
                }
            }
        }

        // Convert the hashmap into a vector of vectors
        let mut res: Vec<Vec<String>> = Vec::new();
        for (_key, val) in map.into_iter() {
            res.push(val);
        }

        res
    }

    /// Sorts the characters of a string and returns the sorted string.
    ///
    /// # Arguments
    ///
    /// * `str` - A reference to the string to be sorted.
    ///
    /// # Returns
    ///
    /// A string containing the characters of the input string sorted in ascending order.
    fn sort_str(str: &String) -> String {
        let mut chars = str.chars().collect::<Vec<char>>(); // Convert the string into a vector of characters
        chars.sort(); // Sort the characters
        chars.into_iter().collect() // Collect the sorted characters into a string
    }
}

pub fn run_tests() {
    let result = Solution::group_anagrams(vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned(),
    ]);

    let expected = vec![
        vec!["bat".to_owned()],
        vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()],
        vec!["tan".to_owned(), "nat".to_owned()],
    ];

    assert_eq!(result.len(), expected.len());

    for group in result {
        assert!(
            expected.contains(&group),
            "Expected group not found: {:?}",
            group
        );
    }
}
