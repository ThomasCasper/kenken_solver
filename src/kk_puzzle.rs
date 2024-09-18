//! kk_puzzle is part of kenken_solve and provides the representation of the puzzle to be solved
//!
//! A puzzle consists of
//!  * the type of the puzzle, i.e. KenKen or Sudoku
//!  * the dimension (3 to 9) of the puzzle (for sudoku this is always 9)
//!  * a field, representing a representation of all set group-solutions
//!  * a list of undecided groups (with more than one option left)
//!  * a blocklist, holding blocklisted digits for each field position
//!
use std::collections::HashSet;
use std::fmt;
use colored::*;

use crate::kk_block_list::BlockList;
use crate::kk_group::Group;
use crate::kk_load::GameType;
use crate::kk_load::GameType::Sudoku;
use crate::kk_load::PuzzleAsString;

#[derive(Debug, Clone,Getters)]
pub struct Puzzle {
    game_type: GameType,
    dimension: usize,
    normal_group_direction:bool,
    solution: Vec<usize>,
    block_list: BlockList,
    groups: Vec<Group>,
}

impl Puzzle {
    /// Copies a puzzle without the groups, i.e., the group list of the new puzzle is empty
    pub fn copy_without_groups(old_field: &Puzzle) -> Self {
        Puzzle {
            game_type: old_field.game_type,
            dimension: old_field.dimension,
            normal_group_direction:old_field.normal_group_direction,
            solution: old_field.solution.clone(),
            block_list: old_field.block_list.clone(),
            groups: Vec::new(),
        }
    }

    pub fn new_from_puzzle_file(puzzle_file: PuzzleAsString) -> Result<Self, String> {
        let mut new_puzzle = Puzzle {
            game_type: *puzzle_file.game_type(),
            dimension: puzzle_file.get_dimension()?,
            normal_group_direction: *puzzle_file.normal_group_direction(),
            solution: vec![0; 90],
            block_list: BlockList::new(),
            groups: Vec::new(),
        };

        if new_puzzle.game_type == Sudoku {
            new_puzzle.initialize_sudoku_from_definition(puzzle_file.puzzle_string())?;
        } else {
            new_puzzle.initialize_kenken_from_definition(puzzle_file.puzzle_string())?;
        }

        Ok(new_puzzle)
    }

    fn initialize_sudoku_from_definition(
        &mut self,
        definition: &Vec<String>,
    ) -> Result<&str, String> {
        //derive field from input strings
        //remember for addressing each row contains 10 digits, hence the join with a 0
        //the length of the field must be 89 = 8*10+9
        self.solution = definition
            .join("0")
            .replace(".", "")
            .replace("-", "0")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        if self.solution.len() != 89 {
            return Err(format!("No valid Sudoku found.\n{:?}", self.solution));
        };

        for quadrant in 0..9 {
            let mut constants: HashSet<usize> = HashSet::new();
            let mut positions: Vec<usize> = Vec::new();
            //fetch constants and open positions of each quadrant
            for i in 0..9 {
                let pos: usize =
                    (3 * (quadrant / 3) + (i / 3)) * 10 + (3 * (quadrant % 3) + (i % 3));
                if self.solution[pos] == 0 {
                    //open field for the group
                    positions.push(pos);
                } else {
                    //found constant
                    constants.insert(self.solution[pos]);
                }
            }
            //add a new group for the open positions
            if positions.len() > 0 {
                let group = Group::new_sudoku(&positions, &constants);
                if group.is_err() {
                    return Err(format!("Quadrant with no valid options found {}", quadrant));
                } else {
                    self.groups.push(group?);
                }
            }
        }

        Ok("ok")
    }

    fn initialize_kenken_from_definition(
        &mut self,
        puzzle_string_vector: &Vec<String>,
    ) -> Result<&str, String> {
        for group_as_string in puzzle_string_vector {
            self.groups
                .push(Group::new_kenken(self.dimension, group_as_string,self.normal_group_direction)?);
        }

        //initialize blocklist and apply first unique digits
        let (o_field, c) = self.get_next_solution_step();

        if let Some(of) = o_field {
            self.solution = of.solution.clone();
            self.block_list = of.block_list.clone();
            self.groups = of.groups.clone();
            self.groups.push(c.unwrap()); //add the best group to groups
        }

        Ok("ok")
    }

    /// Validates the groups of a puzzle against a given field
    /// adds all options with no choices left, i.e. only one option was available
    /// returns
    /// * an option of new puzzle with all undecided groups with the still available options per group,
    /// * an Option of the group for the next try, i.e.
    ///   with the best ratio between available options per size of the group positions
    ///   the group for the next try is not part of the returned new field
    /// if no new Puzzle is returned, the current puzzle is not solvable, i.e. error and next try
    /// if no new option is returned, the puzzle is solved

    pub fn get_next_solution_step(&self) -> (Option<Self>, Option<Group>) {
        let mut new_field = Puzzle::copy_without_groups(&self);
        let mut new_groups = self.groups.clone();
        let mut index: usize = 0;

        let mut ind_min: usize = 0;

        let mut min_opt: usize = 1000;
        let mut min_opt_pos: usize = 1;

        while index < new_groups.len() {
            let (opt_cnt, group_pos, valid_group) = new_groups
                .remove(index)
                .get_updated_group(&new_field.solution, &mut new_field.block_list);

            match opt_cnt {
                // no valid options left ⇒ Error and next try
                0 => {
                    return (None, None);
                }
                // only 1 option left ⇒ Add option (first) to field and restart update
                1 => {
                    valid_group.apply_option_to_field(&mut new_field.solution, 0); //{
                    min_opt = 1000;
                    min_opt_pos = 1;
                    index = 0;
                }
                // more than 1 option left, add group back to list and move to next group
                // if options per positions is better, save this group as the next one to try
                c => {
                    new_groups.insert(index, valid_group);

                    if c * min_opt_pos < min_opt * group_pos {
                        min_opt = opt_cnt;
                        min_opt_pos = group_pos;
                        ind_min = index;
                    };
                    index += 1;
                }
            }
        }

        if new_groups.len() > 0 {
            let best_option = new_groups.remove(ind_min);
            new_field.groups = new_groups;
            (Some(new_field), Some(best_option))
        } else {
            (Some(new_field), None)
        }
    }

    pub fn set_option_for_group(&mut self, group: &Group, option_index: usize) {
        group.apply_option_to_field(&mut self.solution, option_index)
    }

    /// KenKen_solve is the recursive trial and error solver for the puzzles
    /// it accepts the iteration-depth and the current state of the solved puzzle
    ///
    /// the solution is done in the following steps
    ///
    /// * check all groups for valid options in the given solution state
    /// * fill in all groups with only one option left
    /// * if there are still groups with more than 1 option left
    /// * choose and set an option from one of the groups with the best relation of available options and positions
    /// and restart the recursion, if the chosen option for the group was wrong, choose the next option ...
    ///
    pub fn solve(&self) -> Option<Puzzle> {
        let (updated_field_option, next_group_option) = self.get_next_solution_step();

        if next_group_option.is_none() {
            // if no next option available recursion ends
            // if field is None, there was an error
            // otherwise field contains the found solution
            return updated_field_option;
        };

        let next_group = next_group_option.unwrap();
        let updated_field = updated_field_option.unwrap();

        let mut next_field: Puzzle = updated_field.clone();

        for option_index in 0..next_group.options().len() {
            next_field.set_option_for_group(&next_group, option_index);
            if let Some(field) = next_field.solve() {
                return Some(field);
            };
            next_field = updated_field.clone();
        }

        None
    }
}

/// Implementation of the format trait for the puzzle
/// The field is printed as a dimension x dimension matrix
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dimension = self.dimension;
        let display: String = (0..89)
            .map(|index| {
                if (index % 10) < dimension && (index / 10) < dimension {
                    self.solution[index].to_string()
                } else if (index % 10) == dimension && (index / 10) < dimension {
                    "\n".to_string()
                } else {
                    "".to_string()
                }
            })
            .collect();

        write!(f, "{}", display.blue())
    }
}

#[cfg(test)]
mod kk_group_tests {
    use crate::kk_load::GameType::KenKen;

    use super::*;

    #[test]
    //checks the functions
    // * new_from_puzzle_file,
    // * apply_option_to_field and
    // * get_new_valid_field
    //
    // for the kenken KK-DIM4-1.txt example
    fn check_kenken_base() {
        //load a kenken
        let kenken_as_string = PuzzleAsString::new_from_file("KK-Dim4-1.txt").unwrap();

        //check new_from_puzzle_file for kenken
        let kenken_option = Puzzle::new_from_puzzle_file(kenken_as_string);
        assert_eq!(kenken_option.is_ok(), true);
        let mut kenken = kenken_option.unwrap();
        assert_eq!(kenken.game_type, KenKen);
        assert_eq!(kenken.dimension, 4);
        assert_eq!(kenken.groups.len(), 6);
        assert_eq!(kenken.solution.len(), 90);

        //check apply option_to field
        let group = kenken.groups.remove(1);
        kenken.set_option_for_group(&group, 0);
        assert_eq!(kenken.solution[0], 0);
        assert_eq!(kenken.solution[33], 0);
        assert_eq!(kenken.solution[2], 1);
        assert_eq!(kenken.solution[3], 3);
        assert_eq!(kenken.solution[12], 4);
        kenken.set_option_for_group(&group, 1);

        //check get new valid field
        // option nr 1 leads to no solution
        let (new_field_option, next_group_option) = kenken.get_next_solution_step();
        assert_eq!(new_field_option.is_none(), true);
        assert_eq!(next_group_option.is_none(), true);

        //option 5 leads to a solution
        kenken.set_option_for_group(&group, 5);
        let (new_field_option, next_group_option) = kenken.get_next_solution_step();

        assert_eq!(new_field_option.is_some(), true);
        assert_eq!(next_group_option.is_some(), true);

        //apply option 0 from next option for the next solution step
        let mut new_field = new_field_option.unwrap();

        new_field.set_option_for_group(&next_group_option.unwrap(), 0);

        let (new_field_option, next_group_option) = new_field.get_next_solution_step();
        //solution found
        assert_eq!(new_field_option.is_some(), true);
        assert_eq!(next_group_option.is_none(), true);

        //check that found solution is correct
        let found_solution: Vec<usize> = new_field_option
            .unwrap()
            .solution
            .into_iter()
            .filter(|&d| d > 0)
            .collect();

        let manual_solution: Vec<usize> = "2341123434124123"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        assert_eq!(found_solution, manual_solution);
    }

    #[test]
    //checks the functions
    // * new_from_puzzle_file,
    // * apply_option_to_field and
    // * get_new_valid_field
    //
    // for the kenken KK-DIM4-1.txt example
    fn check_kenken_solve() {
        //load a kenken
        let kenken_as_string = PuzzleAsString::new_from_file("KK-Dim9-1.txt").unwrap();

        //check new_from_puzzle_file for kenken
        let kenken_option = Puzzle::new_from_puzzle_file(kenken_as_string);
        assert_eq!(kenken_option.is_ok(), true);
        let kenken = kenken_option.unwrap();
        assert_eq!(kenken.game_type, KenKen);
        assert_eq!(kenken.dimension, 9);
        assert_eq!(kenken.groups.len(), 28);
        assert_eq!(kenken.solution.len(), 90);

        let solution_option = kenken.solve();
        assert_eq!(solution_option.is_some(), true);

        let solution = solution_option.unwrap();
        let found_solution: Vec<usize> = solution
            .solution
            .iter()
            .filter(|&d| d > &0)
            .map(|&d| d)
            .collect();

        let manual_solution: Vec<usize> =
            "473958162529634781618542973892715436781293645147326859934861527356479218265187394"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();

        assert_eq!(found_solution, manual_solution);
    }
}
