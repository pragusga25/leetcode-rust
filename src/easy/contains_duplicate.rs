use super::Solution;
// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // HashMap to store the count of occurrences of each number
        let mut nums_map: HashMap<i32, i32> = HashMap::new();

        // Iterate through each number in the input vector
        for num in nums.into_iter() {
            // Check if the current number exists in the map
            let val = nums_map.get(&num);
            if val.is_some() {
                return true; // Return true if the number is found (duplicate)
            }
            // Insert the number into the map with a count of 1
            nums_map.insert(num, 1);
        }

        false // Return false if no duplicates are found
    }
}

pub fn run_tests() {
    assert_eq!(Solution::contains_duplicate(vec![1, 1, 2, 3]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 4, 2, 3]), false);
    assert_eq!(Solution::contains_duplicate(vec![]), false);
}
