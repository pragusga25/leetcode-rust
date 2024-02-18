use super::Solution;
// https://leetcode.com/problems/generate-parentheses/

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate("".to_owned(), n, n, n)
    }

    fn generate(builder: String, n: i32, openers: i32, closers: i32) -> Vec<String> {
        if openers > n || openers < 0 || closers > n || closers < 0 || closers < openers {
            return vec![];
        }

        if openers == 0 && closers == 0 {
            return vec![builder];
        }

        let mut builder1 = builder.to_owned();
        builder1.push_str("(");
        let mut par1 = Self::generate(builder1, n, openers - 1, closers);

        let mut builder2 = builder.to_owned();
        builder2.push_str(")");
        let par2 = Self::generate(builder2, n, openers, closers - 1);

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
