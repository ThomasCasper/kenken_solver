//! kk_black_list is part of kenken_solve and provides the implementation of a simple black_list
//! for a given position
//!
//! The blacklist consists of a HashMap mapping a position with a HashSet containing the
//! black list for this position. the black list contains digits not allowed on the corresponding
//! position
//!

use std::collections::{HashSet,HashMap};

#[derive(Debug,Clone)]
pub struct BlackList {
    black_list: HashMap<usize,HashSet<usize>>
}

impl BlackList {

    /// Creates a new black list with an empty HashMap, i.e. no blacklisted digits
    /// for no position
    pub fn new() -> Self {
        BlackList {
            black_list: HashMap::new()
        }

    }

    /// Retrieves a blacklist for the given position
    /// If no blacklist is available for the position, an empty blacklist is returned.
    pub fn get_position_black_list(&self, position: &usize) -> HashSet<usize> {
        if let Some(position_black_list) = self.black_list.get(position) {
            position_black_list.clone()
        } else {
            HashSet::<usize>::new()
        }

    }

    /// Adds the given digits to the blacklist of all positions in the same row respectively
    /// same column derived from the given positions of a one-dimensional group
    pub fn insert_position_black_list(&mut self, positions:&Vec<usize>, digits: &HashSet<usize>) {

        let positions_to_update: Vec<usize>;
        let positions_as_hashset:HashSet<usize>= positions.clone().into_iter().collect();

        let column = positions[0] % 10;
        let row= positions[0]- column;

        //get position to update in blacklist
        if column == positions[1]%10 {
            //Dimension: column
            positions_to_update = (column..90).step_by(10)
                .filter(|p| !positions_as_hashset.contains(p)) //get rid of given positions
                .collect();
        } else {
            //Dimension: row
            positions_to_update = (row..row+9)
                .filter(|p| !positions_as_hashset.contains(p)) //get rid of given positions
                .collect();
        }
        for position_to_update in positions_to_update {
            let mut new_position_black_list:HashSet<usize> =digits.clone();

            //join old and new digits
            new_position_black_list
                .extend(self.get_position_black_list(&position_to_update));
            drop(self.black_list.insert(position_to_update, new_position_black_list));
        }

    }
}