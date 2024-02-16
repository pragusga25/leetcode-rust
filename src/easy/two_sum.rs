use super::Solution;
// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // HashMap to store the difference between target and each number along with their indices
        let mut map: HashMap<i32, i32> = HashMap::new();

        // Iterate through each number in the input vector
        for (i, num) in nums.into_iter().enumerate() {
            // Check if the current number's complement exists in the map
            let val = map.get(&num);
            match val {
                Some(val) => return vec![*val, i as i32], // Return indices of the two numbers if found
                None => {
                    // Insert the difference between target and the current number along with its index
                    map.insert(target - num, i as i32);
                }
            }
        }

        // Return [-1, -1] if no solution is found (impossible!)
        vec![-1, -1]
    }
}

pub fn run_tests() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
