//! The module kk_block_list is part of kenken_solve
//! and provides the implementation of a simple block_list for a given position
//!
//! The blocklist consists of a HashMap mapping a position with a HashSet containing the
//! blocklist for this position.
//! The blocklist contains digits not allowed in the corresponding position
//!

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct BlockList {
    block_list: HashMap<usize, HashSet<usize>>,
}

impl BlockList {
    /// Creates a new blocklist with an empty HashMap, i.e., no blocklisted digits
    /// for no position
    pub fn new() -> Self {
        BlockList {
            block_list: HashMap::new(),
        }
    }

    /// Retrieves a blocklist for the given position
    /// If no blocklist is available for the position, an empty blocklist is returned.
    pub fn get_position_block_list(&self, position: &usize) -> HashSet<usize> {
        if let Some(position_block_list) = self.block_list.get(position) {
            position_block_list.clone()
        } else {
            HashSet::<usize>::new()
        }
    }

    /// Checks the given options of a not yet blocklisted (one dimensional) group
    /// if only the same digits are valid, and if so updates the blocklist

    pub fn check_options_and_update_block_list(
        &mut self,
        positions: &Vec<usize>,
        options: &Vec<Vec<usize>>,
    ) -> bool {
        let check_digits: HashSet<usize> = options[0].iter().map(|&digit| digit).collect();

        //check if any of the other options contain any digit not in the first option
        if !options
            .iter()
            .skip(1)
            .any(|option| option.iter().any(|digit| !check_digits.contains(digit)))
        {
            //all available options have the same digits
            //update the blocklist
            self.insert_position_block_list(&positions, &check_digits);
            true
        } else {
            false
        }
    }

    /// Adds the given digits to the blocklist of all positions in the same row respectively
    /// same column derived from the given positions of a one-dimensional group
    fn insert_position_block_list(&mut self, positions: &Vec<usize>, digits: &HashSet<usize>) {
        let positions_to_update: Vec<usize>;
        let positions_as_hashset: HashSet<usize> = positions.clone().into_iter().collect();

        let column = positions[0] % 10;
        let row = positions[0] - column;

        //get position to update in blocklist
        if column == positions[1] % 10 {
            //Dimension: column
            positions_to_update = (column..90)
                .step_by(10)
                .filter(|p| !positions_as_hashset.contains(p)) //get rid of given positions
                .collect();
        } else {
            //Dimension: row
            positions_to_update = (row..row + 9)
                .filter(|p| !positions_as_hashset.contains(p)) //get rid of given positions
                .collect();
        }
        for position_to_update in positions_to_update {
            let mut new_position_block_list: HashSet<usize> = digits.clone();

            //join old and new digits
            new_position_block_list.extend(self.get_position_block_list(&position_to_update));
            drop(
                self.block_list
                    .insert(position_to_update, new_position_block_list),
            );
        }
    }
}

#[cfg(test)]
mod kk_block_list_tests {
    use super::*;

    #[test]
    fn check_new_block_list() {
        let block_list = BlockList::new();
        assert_eq!(block_list.block_list.len(), 0);
    }

    #[test]
    fn check_insert_position_block_list() {
        let mut block_list = BlockList::new();

        //A - row 1
        let positions = vec![10, 11, 12];
        let digits: HashSet<usize> = vec![3, 5, 7].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);
        assert_eq!(block_list.block_list.len(), 6); //#9 columns -3 positions;

        //B - column 2
        let positions = vec![2, 12];
        let digits: HashSet<usize> = vec![4, 6].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);
        assert_eq!(block_list.block_list.len(), 13); //#9 rows - 2 positions + 6 old ones

        //C - column 6
        let positions = vec![36, 46, 56, 66];
        let digits: HashSet<usize> = vec![1, 2, 8, 9].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);
        assert_eq!(block_list.block_list.len(), 17); //#9 rows - 4 positions -1 cross + 13 old ones

        //D - row 4
        let positions = vec![43, 44, 45];
        let digits: HashSet<usize> = vec![3, 4, 7].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);
        assert_eq!(block_list.block_list.len(), 22); //#9 rows - 3 positions -1 cross + 17 old ones

        //normal pos in row 1 ⇒ 3 entries from A
        assert_eq!(block_list.block_list.get(&13).unwrap().len(), 3);
        //normal pos in column 2 ⇒ 2 entries from B
        assert_eq!(block_list.block_list.get(&52).unwrap().len(), 2);
        //normal pos in column 6 ⇒ 4 entries from C
        assert_eq!(block_list.block_list.get(&76).unwrap().len(), 4);
        //normal pos in row 4 ⇒ 3 entries from D
        assert_eq!(block_list.block_list.get(&48).unwrap().len(), 3);

        //cross pos of A and B ⇒ no entries
        assert_eq!(block_list.block_list.get(&12).is_none(), true);
        //cross pos of A and C ⇒ 3+4 entries
        assert_eq!(block_list.block_list.get(&16).unwrap().len(), 7);
        //cross pos of D and B ⇒ 2+3 entries from A and B - 1 Entry overlapping
        assert_eq!(block_list.block_list.get(&42).unwrap().len(), 4);
        //cross pos of D and C ⇒ 3 entries
        assert_eq!(block_list.block_list.get(&46).unwrap().len(), 3);
    }

    #[test]
    fn check_get_position_block_list() {
        let mut block_list = BlockList::new();

        let positions = vec![10, 11, 12];
        let digits: HashSet<usize> = vec![3, 5, 7].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);
        let positions = vec![27, 37, 37, 47];
        let digits: HashSet<usize> = vec![1, 2, 7, 8].into_iter().collect();
        block_list.insert_position_block_list(&positions, &digits);

        assert_eq!(block_list.get_position_block_list(&1).len(), 0);
        assert_eq!(block_list.get_position_block_list(&13).len(), 3);
        assert_eq!(block_list.get_position_block_list(&67).len(), 4);
        assert_eq!(block_list.get_position_block_list(&17).len(), 6); //3 + 4 -1
    }
}
