use super::Solution;
// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let new_digits: Vec<char> = digits.chars().into_iter().collect();
        Self::gen_letter_combinations(&map, &new_digits, "".to_owned(), 0)
    }

    fn gen_letter_combinations(
        map: &HashMap<char, Vec<char>>,
        digits: &Vec<char>,
        builder: String,
        start: usize,
    ) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        if start > digits.len() - 1 {
            return vec![builder];
        }

        let mut res: Vec<String> = vec![];
        let d = digits.get(start).unwrap();

        for c in map.get(&d).unwrap() {
            let mut new_builder = builder.to_owned();
            new_builder.push_str(&*c.to_string());
            res.extend(Self::gen_letter_combinations(
                map,
                digits,
                new_builder,
                start + 1,
            ));
        }

        res
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::letter_combinations("23".to_owned()),
        vec![
            "ad".to_owned(),
            "ae".to_owned(),
            "af".to_owned(),
            "bd".to_owned(),
            "be".to_owned(),
            "bf".to_owned(),
            "cd".to_owned(),
            "ce".to_owned(),
            "cf".to_owned()
        ]
    );
    assert_eq!(
        Solution::letter_combinations("2".to_owned()),
        vec!["a".to_owned(), "b".to_owned(), "c".to_owned(),]
    );
}
