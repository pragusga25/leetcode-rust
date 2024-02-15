use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums_map: HashMap<i32, i32> = HashMap::new();

        for num in nums.into_iter() {
            let val = nums_map.get(&num);

            if val.is_some() {
                return true;
            }
            nums_map.insert(num, 1);
        }

        false
    }
}

pub fn run_tests() {
    assert_eq!(Solution::contains_duplicate(vec![1, 1, 2, 3]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 4, 2, 3]), false);
    assert_eq!(Solution::contains_duplicate(vec![]), false);
}
