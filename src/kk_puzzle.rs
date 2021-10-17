//! kk_puzzle is part of kenken_solve and provides the representation of the puzzle to be solved
//!
//! A puzzle consist of
//!  * the type of the puzzle, i.e. KenKen or Sudoku
//!  * the dimension (3 to 9) of the puzzle (for sudoku this is always 9)
//!  * a field, representing a representation of all set group-solutions
//!  * a list of undecided groups (with more than one option left)
//!  * a blacklist, holding blacklisted digits for each field position
//!
use crate::kk_group::{Group};
use std::fmt;
use crate::kk_load::GameType::{Sudoku};
use crate::kk_load::GameType;
use std::collections::HashSet;
use crate::kk_black_list::BlackList;
use crate::kk_load::PuzzleAsString;


#[derive(Debug,Clone)]
pub struct Puzzle {
    game_type: GameType,
    dimension: usize,
    field:Vec<usize>,
    black_list:BlackList,
    groups:Vec<Group>
}


impl Puzzle {

    /// Copies a puzzle without the groups, i.e. the list of groups of the new puzzle is empty
    pub fn copy_without_groups(old_field: &Puzzle) -> Self {

        Puzzle {
            game_type: old_field.game_type,
            dimension: old_field.dimension,
            field: old_field.field.clone(),
            black_list:old_field.black_list.clone(),
            groups: Vec::new(),
        }

    }

    pub fn new_from_puzzle_file( puzzle_file: PuzzleAsString) -> Result<Self, String> {
        let mut new_puzzle=Puzzle {
            game_type: *puzzle_file.game_type(),
            dimension: puzzle_file.get_dimension()?,
            field: vec![0; 90],
            black_list: BlackList::new(),
            groups: Vec::new(),
        };

        if new_puzzle.game_type == Sudoku {
            new_puzzle.initialize_sudoku_from_definition(puzzle_file.puzzle_string())?;
        } else {
            new_puzzle.initialize_kenken_from_definition(puzzle_file.puzzle_string())?;
        }

        Ok(new_puzzle)
    }

    fn initialize_sudoku_from_definition(&mut self, definition: &Vec<String>) -> Result<&str, String> {
        //derive field from input strings
        //remember for addressing each row contains 10 digits, hence the join with a 0
        //the length of the field must be 89 = 8*10+9
        self.field = definition.join("0")
            .replace(".","")
            .replace("-","0")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        if self.field.len() != 89 {
            return Err(format!("No valid Sudoku found.\n{:?}", self.field));
        };

        for quadrant in 0..9 {

            let mut constants:HashSet<usize>=HashSet::new();
            let mut positions:Vec<usize>=Vec::new();
            //fetch constants and open positions of each quadrant
            for i in 0..9 {
                let pos:usize = (3*(quadrant/3)+(i/3))*10+(3*(quadrant % 3) + (i % 3));
                if self.field[pos] == 0 {
                    //open field for cell
                    positions.push(pos);
                } else {
                    //found constant
                    constants.insert(self.field[pos]);
                }
            }
            //add a new cell for the open positions
            if positions.len()>0 {
                let group = Group::new_sudoku(&positions, &constants);
                 if group.is_err() {
                    return Err(format!("Quadrant with no valid options found {}", quadrant));
                }
                self.groups.push(group.unwrap());
            }
        }

        Ok("ok")
    }

    fn initialize_kenken_from_definition(&mut self, puzzle_string_vector: &Vec<String>) -> Result<&str, String> {

        for cell_as_string in puzzle_string_vector {
            self.groups.push(Group::new_kenken(self.dimension,cell_as_string)?);
        }


        //initialize blacklist and apply first unique digits
        let (o_field,c)= self.get_new_valid_field();

        if let Some(of)=o_field {
            self.field = of.field.clone();
            self.black_list = of.black_list.clone();
            self.groups = of.groups.clone();
            self.groups.push(c.unwrap());  //add best cell to cells
        }

        Ok("ok")
    }



    /// Validates the cells of a field against a given field
    /// adds all options with no choices left, i.e. only one option was available
    /// returns a new field with all undecided cell with the open options,
    /// a count of the undecided cells and the next Cell to get "tried"
    /// the cell for the net try has the shortest possible length of open options
    /// the cell for the next try is not part of the returned new field
    /// if the count is 0, no Cell will be returned
    /// if count is 0, and a field is returned: The Kenken was solved and the returned field is the solution
    /// if count is 0 and the field is None, there where no valid options left and the try was an error

    pub fn get_new_valid_field(&self) -> (Option<Self>, Option<Group>) {
        let mut new_field = Puzzle::copy_without_groups(&self);
        let mut new_cells = self.groups.clone();
        let mut index:usize = 0;

        let mut ind_min:usize=0;

        let mut min_opt:usize=1000;
        let mut min_opt_pos:usize=1;
        //println!("New validation: {}", new_cells.len());
        while index < new_cells.len() {
            //println!("{} - {}",ind, new_cells.len());
            let (opt_cnt, cell_pos,valid_cell) = new_cells.remove(index)
                .get_valid_options(&new_field.field, &mut new_field.black_list);

            match opt_cnt {
                // no valid options left => Error and next try
                0 => {
                    //println!("Cell with count 0: {} - {:?}",ind,valid_cell);
                    //println!("New field with cnt 0: {:?}", new_field);
                    return (None, None);
                },
                // only 1 option left => Add option (first) to field and restart update
                1 => {

                    valid_cell.apply_option_to_field(&mut new_field.field, 0); //{
                    min_opt = 1000;
                    min_opt_pos =1;
                    index = 0;


                },
                // more than 1 option left, add cell back to list and move to next cell
                // if options per positions is better, save this cell as the next one to try
                c => {
                    new_cells.insert(index,valid_cell);

                    if c*min_opt_pos<min_opt*cell_pos {

                        min_opt=opt_cnt;
                        min_opt_pos=cell_pos;
                        ind_min=index;
                    };
                    index+=1;
                }
            }
        }

        if new_cells.len()>0 {
            let best_option= new_cells.remove(ind_min);
            new_field.groups = new_cells;
            (Some(new_field),Some(best_option))
        }
        else {
            (Some(new_field),None)
        }

    }

    pub fn apply_option_to_field(&mut self, cell: &Group, option_nr: usize) -> bool {

        cell.apply_option_to_field(& mut self.field, option_nr)

    }

    /// KenKen_solve is the recursive try and error solver for the puzzles
/// it accepts the iteration-depth and the current state of the solved puzzle
///
/// the solution is done in the following steps
///
/// * check all cells for valid options in the given solution state
/// * fill in all cells with only one option left
/// * if there are still cell with more than 1 option left
/// * choose and set an option from one of the cells with the less most available options
/// and restart the recursion, if the chosen option for the cell was wrong, choose the next option ...
///
    pub fn solve(&self) -> Option<Puzzle> {
        let (updated_field_option, next_cell_option) = self.get_new_valid_field();

        if next_cell_option.is_none(){
            // if no next option available recursion ends
            // if field is None there was an error
            // otherwise field contains the found solution
            return updated_field_option;
        };

        let next_cell = next_cell_option.unwrap();
        let updated_field = updated_field_option.unwrap();

        let mut current_option: usize = 0;

        let mut next_field: Puzzle = updated_field.clone();

        while next_field.apply_option_to_field(&next_cell, current_option) {
            current_option += 1;
            if let Some(field) = next_field.solve() {
                return Some(field);
            };
            next_field = updated_field.clone();
        };


        None
    }


}

/// Implementation of the format trait for the puzzle
/// The field is printed as a dimension x dimension matrix
impl fmt::Display for Puzzle {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dimension = self.dimension;
        let display:String = (0..89)
            .map(|index|
                if (index % 10) < dimension && (index / 10) < dimension {
                    self.field[index].to_string()
                } else if (index % 10) == dimension && (index / 10) < dimension {
                    "\n".to_string()
                } else {
                    "".to_string()
                })
            .collect();

        write!(f, "{}", display)
    }

}

#[cfg(test)]
mod kk_group_tests {
    use super::*;
    use crate::kk_load::GameType::KenKen;

    #[test]
    //checks the functions
    // * new_from_puzzle_file,
    // * apply_option_to_field and
    // * get_new_valid_field
    //
    // for the kenken KK-DIM4-1.txt example
    fn check_kenken_base() {
        //load a kenken
        let kenken_as_string= PuzzleAsString::new_from_file("KK-Dim4-1.txt").unwrap();

        //check new_from_puzzle_file for kenken
        let kenken_option = Puzzle::new_from_puzzle_file(kenken_as_string);
        assert_eq!(kenken_option.is_ok(), true);
        let mut kenken = kenken_option.unwrap();
        assert_eq!(kenken.game_type, KenKen);
        assert_eq!(kenken.dimension, 4);
        assert_eq!(kenken.groups.len(), 6);
        assert_eq!(kenken.field.len(), 90);

        //check apply option_to field
        let group=kenken.groups.remove(1);
        assert_eq!(kenken.apply_option_to_field(&group,0),true);
        assert_eq!(kenken.field[0],0);
        assert_eq!(kenken.field[33],0);
        assert_eq!(kenken.field[2],1);
        assert_eq!(kenken.field[3],3);
        assert_eq!(kenken.field[12],4);
        assert_eq!(kenken.apply_option_to_field(&group,8),false);
        assert_eq!(kenken.apply_option_to_field(&group,1),true);

        //check get new valid field
        // option nr 1 leads to no solution
        let (new_field_option, next_group_option)=kenken.get_new_valid_field();
        assert_eq!(new_field_option.is_none(), true);
        assert_eq!(next_group_option.is_none(), true);

        //option 5 leads to a solution
        assert_eq!(kenken.apply_option_to_field(&group,5),true);
        let (new_field_option, next_group_option)=kenken.get_new_valid_field();

        assert_eq!(new_field_option.is_some(), true);
        assert_eq!(next_group_option.is_some(), true);

        //apply option 0 from next option for the next solution step
        let mut new_field=new_field_option.unwrap();
        assert_eq!(new_field.apply_option_to_field(&next_group_option.unwrap(),0),true);

        let (new_field_option, next_group_option)=new_field.get_new_valid_field();
        //solution found
        assert_eq!(new_field_option.is_some(), true);
        assert_eq!(next_group_option.is_none(), true);

        //check that found solution is correct
        let found_solution:Vec<usize>=new_field_option.unwrap()
            .field.into_iter()
            .filter(|&d| d>0)
            .collect();

        let manual_solution:Vec<usize>="2341123434124123"
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
         assert_eq!(kenken.field.len(), 90);

         let solution_option=kenken.solve();
         assert_eq!(solution_option.is_some(), true);

         let solution = solution_option.unwrap();
         let found_solution:Vec<usize>=solution
            .field.iter()
            .filter(|&d| d>&0)
             .map(|&d| d)
            .collect();

        let manual_solution:Vec<usize>="473958162529634781618542973892715436781293645147326859934861527356479218265187394"
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        assert_eq!(found_solution, manual_solution);

     }

}