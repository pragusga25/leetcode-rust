use super::Solution;
// https://leetcode.com/problems/generate-parentheses/

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::gen_parenthesis("".to_owned(), n, 0, 0)
    }

    fn gen_parenthesis(builder: String, n: i32, openers: i32, closers: i32) -> Vec<String> {
        if openers > n || closers > n || closers > openers {
            return vec![];
        }

        if openers == n && closers == n {
            return vec![builder];
        }

        let mut builder1 = builder.to_owned();
        builder1.push_str("(");
        let mut par1 = Self::gen_parenthesis(builder1, n, openers + 1, closers);

        let mut builder2 = builder.to_owned();
        builder2.push_str(")");
        let par2 = Self::gen_parenthesis(builder2, n, openers, closers + 1);

        par1.extend(par2);
        par1
    }
}

pub fn run_tests() {
    assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_owned()]);
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec![
            "((()))".to_owned(),
            "(()())".to_owned(),
            "(())()".to_owned(),
            "()(())".to_owned(),
            "()()()".to_owned()
        ]
    );
}
