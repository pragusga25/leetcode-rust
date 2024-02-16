use super::Solution;
// https://leetcode.com/problems/top-k-frequent-elements/

use std::collections::{HashMap, VecDeque};

struct Node {
    num: i32,
    count: u32,
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // HashMap to store frequency of each number
        let mut map: HashMap<i32, u32> = HashMap::new();
        // VecDeque to store nodes with numbers and their frequencies
        let mut heap: VecDeque<Node> = VecDeque::new();

        // Calculate frequencies of numbers and store them in the map
        for num in nums.into_iter() {
            let val = map.get(&num);
            match val {
                Some(val) => {
                    map.insert(num, val + 1);
                }
                None => {
                    map.insert(num, 1);
                }
            }
        }

        // Convert map entries into nodes and push them into the heap
        for (key, value) in map.into_iter() {
            let node = Node {
                count: value,
                num: key,
            };
            heap.push_back(node);
        }

        // Build max heap from the heap
        let len = heap.len();
        let index_start = (((len / 2) as f32).floor() as i32 + 1) as usize;
        for i in (0..=index_start).rev() {
            Self::max_heapify(&mut heap, i);
        }

        // Pop the root node (max frequency) k times and store the numbers in the result
        let mut res: Vec<i32> = Vec::new();
        for _ in 1..=k {
            let node = Self::pop_root(&mut heap);
            match node {
                Some(node) => {
                    res.push(node.num);
                }
                None => {}
            }
        }

        res
    }

    // Pop the root node (max frequency) from the heap
    fn pop_root(heap: &mut VecDeque<Node>) -> Option<Node> {
        let last_element = heap.pop_back();
        if heap.len() == 0 {
            return last_element;
        }

        let front = heap.pop_front();
        match last_element {
            Some(le) => {
                heap.push_front(le);
                Self::max_heapify(heap, 0);
            }
            None => {}
        }

        front
    }

    // Perform max heapify operation on the heap starting from index i
    fn max_heapify(heap: &mut VecDeque<Node>, i: usize) -> () {
        let len = heap.len();
        let max_i_to_heapify = (((len / 2) as f32).floor() as i32 + 1) as usize;
        if i > max_i_to_heapify || i >= len {
            return;
        }

        let l = 2 * i + 1;
        let r = 2 * i + 2;

        let el = heap.get(l);
        let er = heap.get(r);
        let ei = heap.get(i).unwrap();

        match (el, er) {
            (Some(el), Some(er)) => {
                if ei.count > el.count && ei.count > er.count {
                    return;
                }

                if el.count > er.count {
                    heap.swap(i, l);
                    Self::max_heapify(heap, l);
                } else {
                    heap.swap(i, r);
                    Self::max_heapify(heap, r);
                }
            }
            (Some(el), None) => {
                if ei.count > el.count {
                    return;
                }

                heap.swap(i, l);
                Self::max_heapify(heap, l);
            }
            (None, Some(_)) => {
                // it's not possible
            }
            (None, None) => {}
        }
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2).sort(),
        vec![1, 2].sort()
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
