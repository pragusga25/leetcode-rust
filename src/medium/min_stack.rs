use std::collections::VecDeque;

struct MinStack {
    stack: Vec<i32>,
    mindeque: VecDeque<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            mindeque: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.mindeque.is_empty() {
            self.mindeque.push_back(val);
            return;
        }

        let min = *self.mindeque.back().unwrap();
        if val < min {
            self.mindeque.push_back(val);
        } else {
            self.mindeque.push_front(val);
        }
    }

    fn pop(&mut self) {
        let bot = *self.mindeque.front().unwrap();
        let pop = self.stack.pop().unwrap();

        if pop == bot {
            self.mindeque.pop_front();
        } else {
            self.mindeque.pop_back();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mindeque.back().unwrap()
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
