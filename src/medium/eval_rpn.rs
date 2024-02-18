use super::Solution;
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for ch in tokens.into_iter() {
            match &*ch {
                "+" => {
                    let num2 = stack.pop().unwrap();
                    let num1 = stack.pop().unwrap();
                    stack.push(num1 + num2);
                }
                "-" => {
                    let num2 = stack.pop().unwrap();
                    let num1 = stack.pop().unwrap();
                    stack.push(num1 - num2);
                }
                "*" => {
                    let num2 = stack.pop().unwrap();
                    let num1 = stack.pop().unwrap();
                    stack.push(num1 * num2);
                }
                "/" => {
                    let num2 = stack.pop().unwrap();
                    let num1 = stack.pop().unwrap();
                    stack.push(num1 / num2);
                }
                ch => {
                    let num: i32 = ch.parse().unwrap();
                    stack.push(num);
                }
            }
        }
        stack.pop().unwrap()
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::eval_rpn(vec![
            "2".to_owned(),
            "1".to_owned(),
            "+".to_owned(),
            "3".to_owned(),
            "*".to_owned()
        ]),
        9
    );
    assert_eq!(
        Solution::eval_rpn(vec![
            "10".to_owned(),
            "6".to_owned(),
            "9".to_owned(),
            "3".to_owned(),
            "+".to_owned(),
            "-11".to_owned(),
            "*".to_owned(),
            "/".to_owned(),
            "*".to_owned(),
            "17".to_owned(),
            "+".to_owned(),
            "5".to_owned(),
            "+".to_owned()
        ]),
        22
    );
}
