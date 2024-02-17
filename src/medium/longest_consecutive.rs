use super::Solution;
// https://leetcode.com/problems/product-of-array-except-self/

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // HashMap to store the count of consecutive elements starting from each number
        let mut map: HashMap<i32, (i32, bool)> = HashMap::new();
        // HashSet to store unique numbers from the input vector
        let mut set: HashSet<i32> = HashSet::new();

        // Initialize the map and set with numbers from the input vector
        for num in nums.into_iter() {
            map.insert(num, (1, false)); // Initialize count as 1 and is_visited as false
            set.insert(num);
        }

        let mut max: i32 = 0;

        // Iterate through each unique number in the set
        for key in set.into_iter() {
            let val = *map.get(&key).unwrap();
            let (_, is_visited) = val;

            if is_visited {
                continue; // Skip if the number has already been visited
            }

            // Calculate the length of consecutive elements starting from the current number
            let step = Self::next_step(&mut map, key);
            if step > max {
                max = step; // Update the maximum length if needed
            }
        }
        max // Return the maximum length of consecutive elements
    }

    // Recursive function to calculate the length of consecutive elements starting from a given number
    fn next_step(map: &mut HashMap<i32, (i32, bool)>, start: i32) -> i32 {
        let val = map.get(&start);

        match val {
            Some(val) => {
                let (count, is_visited) = *val;
                if is_visited {
                    return count; // Return the count if the number has already been visited
                }

                // Calculate the length of consecutive elements starting from the next number
                let new_count = count + Self::next_step(map, start + 1);
                map.insert(start, (new_count, true)); // Mark the current number as visited
                new_count // Return the length of consecutive elements
            }
            None => 0, // Return 0 if the number is not found in the map
        }
    }
}

pub fn run_tests() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
}
