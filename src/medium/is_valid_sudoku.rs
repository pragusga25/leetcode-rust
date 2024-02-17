use super::Solution;
// https://leetcode.com/problems/product-of-array-except-self/

use std::collections::{HashMap, HashSet};

impl Solution {
    /// Determines if the given Sudoku board is valid.
    ///
    /// The function checks if the board satisfies the Sudoku rules:
    /// each row, column, and 3x3 subgrid must contain the digits 1-9 without repetition.
    ///
    /// # Arguments
    ///
    /// * `board` - A 9x9 grid represented as a vector of vectors of characters.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the Sudoku board is valid or not.
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

                for key in [key_box, key_row, key_col] {
                    let is_valid = Self::is_valid(&mut map, &key, &ch);
                    if !is_valid {
                        return is_valid;
                    }
                }
            }
        }

        true
    }

    /// Helper function to validate a character in a specific row, column, or box.
    ///
    /// It checks if the character already exists in the corresponding set.
    /// If it doesn't exist, it inserts the character into the set.
    ///
    /// # Arguments
    ///
    /// * `map` - A mutable reference to a HashMap storing sets of characters for each key.
    /// * `key` - The key indicating the row, column, or box.
    /// * `ch` - The character to be validated and inserted into the set.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the character is valid and inserted successfully.
    fn is_valid(map: &mut HashMap<String, HashSet<char>>, key: &String, ch: &char) -> bool {
        let set = map.get_mut(key);
        match set {
            Some(set) => {
                if set.contains(&ch) {
                    return false;
                }
                set.insert(*ch);
                true
            }
            None => {
                let mut hash_set: HashSet<char> = HashSet::with_capacity(9);
                hash_set.insert(*ch);
                map.insert(key.to_owned(), hash_set);
                true
            }
        }
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
