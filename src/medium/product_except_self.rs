use super::Solution;
// https://leetcode.com/problems/product-of-array-except-self/

use std::collections::VecDeque;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // Initialize two deques to store products from the left and right sides respectively
        let len = nums.len();
        let mut mfl: VecDeque<i32> = VecDeque::with_capacity(len);
        let mut mfr: VecDeque<i32> = VecDeque::with_capacity(len);

        // Push initial values (1) to the deques to handle edge cases
        mfl.push_back(1);
        mfr.push_back(1);

        // Calculate products from the left and right sides
        for l in 0..len - 1 {
            let r = len - 1 - l;
            let prevl = mfl.back().unwrap();
            let prevr = mfr.front().unwrap();

            let numl = nums.get(l).unwrap();
            let numr = nums.get(r).unwrap();

            mfl.push_back(prevl * numl);
            mfr.push_front(prevr * numr);
        }

        // Calculate the final result by multiplying corresponding elements from both deques
        let mut res: Vec<i32> = Vec::new();

        for i in 0..len {
            let ml = mfl.get(i).unwrap();
            let mr = mfr.get(i).unwrap();
            res.push(ml * mr);
        }

        res
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
