use super::Solution;
// https://leetcode.com/problems/valid-parentheses/

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // HashMap to store closing brackets and their corresponding opening brackets
        let map: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
        // HashSet to store opening brackets
        let open_brackets = HashSet::from(['(', '{', '[']);
        // Stack to store opening brackets encountered
        let mut stack: Vec<char> = Vec::new();

        // Iterate through each character in the string
        for ch in s.chars() {
            if open_brackets.contains(&ch) {
                stack.push(ch); // Push opening brackets onto the stack
                continue;
            }

            let top = stack.pop(); // Pop the top element from the stack
            if top.is_none() {
                return false; // Return false if there are no opening brackets to match
            }

            // Check if the top element matches the corresponding opening bracket for the current closing bracket
            if top.unwrap() != map.get(&ch).unwrap().to_owned() {
                return false; // Return false if there is no matching opening bracket
            }
        }

        stack.is_empty() // Return true if the stack is empty (all opening brackets are matched)
    }
}

pub fn run_tests() {
    assert_eq!(Solution::is_valid("(){}({})".to_owned()), true);
    assert_eq!(Solution::is_valid("(){}({[]})".to_owned()), true);
    assert_eq!(Solution::is_valid("()".to_owned()), true);
    assert_eq!(Solution::is_valid("([]}".to_owned()), false);
    assert_eq!(Solution::is_valid("(".to_owned()), false);
    assert_eq!(Solution::is_valid("".to_owned()), true);
}
