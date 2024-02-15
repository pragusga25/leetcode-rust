use super::Solution;
// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs.into_iter() {
            let sorted_str = Solution::sort_str(&str);
            let val = map.get_mut(&sorted_str);

            match val {
                Some(val) => {
                    val.push(str);
                }
                None => {
                    map.insert(sorted_str, vec![str]);
                }
            }
        }

        let mut res: Vec<Vec<String>> = Vec::new();

        for (_key, val) in map.into_iter() {
            res.push(val);
        }

        res
    }

    fn sort_str(str: &String) -> String {
        let str_slice = &str[..];
        let mut chrs = str_slice.chars().collect::<Vec<char>>();
        chrs.sort_by(|a, b| b.cmp(a));
        chrs.into_iter().collect()
    }
}

pub fn run_tests() {
    let result = Solution::group_anagrams(vec![
        "eat".to_owned(),
        "tea".to_owned(),
        "tan".to_owned(),
        "ate".to_owned(),
        "nat".to_owned(),
        "bat".to_owned(),
    ]);

    let expected = vec![
        vec!["bat".to_owned()],
        vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()],
        vec!["tan".to_owned(), "nat".to_owned()],
    ];

    assert_eq!(result.len(), expected.len());

    for group in result {
        assert!(
            expected.contains(&group),
            "Expected group not found: {:?}",
            group
        );
    }
}
