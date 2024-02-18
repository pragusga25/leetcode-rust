use super::Solution;
// https://leetcode.com/problems/combination-sum/

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Self::gen_combination_sum(0, &candidates, target, &mut res, vec![]);
        res
    }

    fn gen_combination_sum(
        start: usize,
        candidates: &Vec<i32>,
        target: i32,
        res: &mut Vec<Vec<i32>>,
        mut tmp: Vec<i32>,
    ) {
        if target < 0 {
            return;
        }

        if target == 0 {
            res.push(tmp);
            return;
        }

        for i in start..candidates.len() {
            let candidate = candidates.get(i).unwrap();
            tmp.push(*candidate);
            Self::gen_combination_sum(i, candidates, target - candidate, res, tmp.clone());
            tmp.pop();
        }
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}
