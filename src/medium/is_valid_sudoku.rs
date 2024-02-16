use super::Solution;
// https://leetcode.com/problems/product-of-array-except-self/

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // HashMap to store sets of numbers for each row, column, and 3x3 box
        let mut map: HashMap<String, HashSet<char>> = HashMap::new();

        // Iterate through each cell in the board
        for (i, row) in board.into_iter().enumerate() {
            for (j, ch) in row.into_iter().enumerate() {
                // Skip empty cells
                if ch == '.' {
                    continue;
                }

                // Calculate the index of the 3x3 box containing the cell
                let bi = (i / 3) as u8;
                let bj = (j / 3) as u8;

                // Generate keys for the box, row, and column
                let key_box = format!("B_{}{}", bi, bj);
                let key_row = format!("R_{}", i);
                let key_col = format!("C_{}", j);

                // Check if the current number violates the Sudoku rules in the box
                let set_box = map.get_mut(&key_box);
                match set_box {
                    Some(set_box) => {
                        if set_box.contains(&ch) {
                            return false;
                        }
                        set_box.insert(ch);
                    }
                    None => {
                        let mut hash_set: HashSet<char> = HashSet::with_capacity(9);
                        hash_set.insert(ch);
                        map.insert(key_box, hash_set);
                    }
                }

                // Check if the current number violates the Sudoku rules in the row
                let set_row = map.get_mut(&key_row);
                match set_row {
                    Some(set_row) => {
                        if set_row.contains(&ch) {
                            return false;
                        }
                        set_row.insert(ch);
                    }
                    None => {
                        let mut hash_set: HashSet<char> = HashSet::with_capacity(9);
                        hash_set.insert(ch);
                        map.insert(key_row, hash_set);
                    }
                }

                // Check if the current number violates the Sudoku rules in the column
                let set_col = map.get_mut(&key_col);
                match set_col {
                    Some(set_col) => {
                        if set_col.contains(&ch) {
                            return false;
                        }
                        set_col.insert(ch);
                    }
                    None => {
                        let mut hash_set: HashSet<char> = HashSet::with_capacity(9);
                        hash_set.insert(ch);
                        map.insert(key_col, hash_set);
                    }
                }
            }
        }

        true
    }
}

pub fn run_tests() {
    assert_eq!(
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]),
        true
    );
    assert_eq!(
        Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]),
        false
    );
}
