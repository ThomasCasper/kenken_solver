
use crate::kk_cell::{Cell};
use std::fmt;
use crate::kk_load::GameType::{KenKen, Sudoku};
use crate::kk_load::GameType;
use std::collections::HashSet;
use crate::kk_improve::BlackList;
use crate::kk_load::PuzzleAsString;


#[derive(Debug,Clone)]
pub struct Field {
    game_type: GameType,
    dimension: usize,
    field:Vec<usize>,
    black_list:BlackList,
    cells:Vec<Cell>
}


impl Field {
    pub fn new() -> Self {
        Field {
            game_type: KenKen,
            dimension: 0,
            field: vec![0; 100],
            black_list: BlackList::new(),
            cells: Vec::new(),
        }
    }

    pub fn copy_without_cells(old_field: &Field) -> Self {

        Field {
            game_type: old_field.game_type,
            dimension: old_field.dimension,
            field: old_field.field.clone(),
            black_list:old_field.black_list.clone(),
            cells: Vec::new(),
        }

    }

    pub fn initialize_from_puzzle_file(&mut self, puzzle_file: PuzzleAsString) -> Result<&str, String> {

        self.game_type = *puzzle_file.game_type();
        self.dimension = puzzle_file.get_dimension()?;
        if self.game_type == Sudoku {
            self.initialize_sudoku_from_definition(puzzle_file.puzzle_string())
        } else {
            self.initialize_kenken_from_definition(puzzle_file.puzzle_string())
        }
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
                let mut cell = Cell::new(&positions, 's', 45);
                 if cell.add_options_base_sudoku(&constants) == 0 {
                    return Err(format!("Quadrant with no valid options found {}", quadrant));
                }
                self.cells.push(cell);
            }
        }

        Ok("ok")
    }

    fn initialize_kenken_from_definition(&mut self, puzzle_string_vector: &Vec<String>) -> Result<&str, String> {

        for cell_as_string in puzzle_string_vector {
            self.cells.push(Cell::new_from_string(cell_as_string)?);
        }

        //Add options to Cells
        for cell in &mut self.cells {
            if cell.add_options_base_kenken(self.dimension) == 0 {
                return Err(format!("Cell has no valid option - {:?}",cell));
            }
        }

        //initialize blacklist and apply first unique digits
        let (o_field,c)= self.get_new_valid_field();

        if let Some(of)=o_field {
            self.field = of.field.clone();
            self.black_list = of.black_list.clone();
            self.cells = of.cells.clone();
            self.cells.push(c.unwrap());  //add best cell to cells
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

    pub fn get_new_valid_field(&self) -> (Option<Self>, Option<Cell>) {
        let mut new_field = Field::copy_without_cells(&self);
        let mut new_cells = self.cells.clone();
        let mut index:usize = 0;

        let mut ind_min:usize=0;

        let mut min_opt:usize=1000;
        let mut min_opt_pos:usize=1;
        //println!("New validation: {}", new_cells.len());
        while index < new_cells.len() {
            //println!("{} - {}",ind, new_cells.len());
            let (opt_cnt, cell_pos,valid_cell) = new_cells.remove(index)
                .get_valid_cell_options(&new_field.field,&mut new_field.black_list);

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
            new_field.cells = new_cells;
            (Some(new_field),Some(best_option))
        }
        else {
            (Some(new_field),None)
        }

    }

    pub fn apply_option_to_field(&mut self, cell: &Cell, option_nr: usize) -> bool {

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
    pub fn solve(self) -> Option<Field> {
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

        let mut next_field: Field = updated_field.clone();

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




impl fmt::Display for Field {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.dimension;
        let display:String = (0..89)
            .map(|index|
                if (index % 10) < d && (index / 10) < d {
                    self.field[index].to_string()
                } else if (index % 10) == d && (index / 10) < d {
                    "\n".to_string()
                } else {
                    "".to_string()
                })
            .collect();

        write!(f, "{}", display)
    }

}