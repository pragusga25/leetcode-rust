// https://leetcode.com/problems/min-stack/

use std::collections::VecDeque;

struct MinStack {
    stack: Vec<i32>,
    mindeque: VecDeque<i32>,
}

impl MinStack {
    // Initialize a new MinStack
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),         // Stack to store values
            mindeque: VecDeque::new(), // Deque to store minimum values
        }
    }

    // Push a value onto the stack
    fn push(&mut self, val: i32) {
        self.stack.push(val); // Push the value onto the stack
        if self.mindeque.is_empty() {
            self.mindeque.push_back(val); // If the deque is empty, push the value onto it
            return;
        }

        let min = *self.mindeque.back().unwrap(); // Get the current minimum value from the back of the deque
        if val < min {
            self.mindeque.push_back(val); // If the value is smaller than the minimum, push it onto the back of the deque
        } else {
            self.mindeque.push_front(val); // Otherwise, push it onto the front of the deque
        }
    }

    // Pop the top element from the stack
    fn pop(&mut self) {
        let bot = *self.mindeque.front().unwrap(); // Get the current minimum value from the front of the deque
        let pop = self.stack.pop().unwrap(); // Pop the top element from the stack

        if pop == bot {
            self.mindeque.pop_front(); // If the popped value is equal to the current minimum, pop it from the front of the deque
        } else {
            self.mindeque.pop_back(); // Otherwise, pop it from the back of the deque
        }
    }

    // Get the top element of the stack
    fn top(&self) -> i32 {
        *self.stack.last().unwrap() // Get the last element of the stack
    }

    // Get the minimum element of the stack
    fn get_min(&self) -> i32 {
        *self.mindeque.back().unwrap() // Get the last element of the deque, which represents the minimum value
    }
}

pub fn run_tests() {
    let cmds = vec![
        "MinStack", "push", "push", "push", "top", "pop", "getMin", "pop", "getMin", "pop", "push",
        "top", "getMin", "push", "top", "getMin", "pop", "getMin",
    ];
    let input: Vec<Vec<i32>> = vec![
        vec![],
        vec![2147483646],
        vec![2147483646],
        vec![2147483647],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![2147483647],
        vec![],
        vec![],
        vec![-2147483648],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    let mut output: Vec<Option<i32>> = Vec::new();
    let mut min_stack = MinStack::new();
    for (i, cmd) in cmds.into_iter().enumerate() {
        let inp = input.get(i).unwrap().get(0);
        match cmd {
            "push" => {
                min_stack.push(*inp.unwrap());
                output.push(None);
            }
            "top" => {
                let top = min_stack.top();
                output.push(Some(top));
            }
            "getMin" => {
                let min = min_stack.get_min();
                output.push(Some(min));
            }
            "pop" => {
                min_stack.pop();
                output.push(None);
            }
            _ => {
                output.push(None);
            }
        }
    }

    let expected: Vec<Option<i32>> = vec![
        None,
        None,
        None,
        None,
        Some(2147483647),
        None,
        Some(2147483646),
        None,
        Some(2147483646),
        None,
        None,
        Some(2147483647),
        Some(2147483647),
        None,
        Some(-2147483648),
        Some(-2147483648),
        None,
        Some(2147483647),
    ];

    assert_eq!(output, expected);
}
