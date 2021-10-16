//! kk_cells is part of kenken_solve and provides the implementation of a basic cell
//!
//! A cell is the combination of single positions on the kenken field which full fill
//! a given mathematical operation.
//! The Kenken puzzle consists of a set of cells. This set outlays the n x n kenken field
//!
//!
use std::collections::HashSet;
use permutohedron::heap_recursive;
use itertools::Itertools;

use crate::kk_improve::BlackList;

/// Struct cell describes a single cell
/// A cell consists of
/// * a result of the mathematical operation
/// * the mathematical operation ('+', '-', '*', ':' or 'c' for constants)
/// * the (vector of) single positions within the kenken puzzle belonging to the cell
/// * the (vector of) the possible options (solutions) for the cell full filling the operation
///   (the solution contains exactly one option)
/// * a marker, if the cell is one dimensional, i.e. all positions are in exactly one row or column
/// * a marker, if the cell was black listed already, i.e. the options where cleaned from invalid one.
///
#[derive(Debug,Clone)]
pub struct Cell {
    result: usize,
    operation: char,
    positions:Vec<usize>,
    options: Vec<Vec<usize>>,
    pub is_onedim: bool,
    pub is_black_listed: bool
}

impl Cell {
    /// Create new Cell with position, ops and res
    /// There are no options attached
    /// use add_options_base for the initial options based on result and ops
    /// or use add_option for direct option attachment
    pub fn new(new_pos: &Vec<usize>, new_ops: char, new_res: usize) -> Self {

        Cell {
            operation: new_ops,
            result: new_res,
            positions: new_pos.clone(),
            options: Vec::new(),
            is_black_listed: false,
            //check if all positions are in one line or column, if yes
            //the cell is one dimensional
            is_onedim: new_pos.iter()
                        .map(|p| p/10) //row
                        .fold(true, |s, p| s && new_pos[0]/10==p) ||
                        new_pos.iter()
                        .map(|p| p%10) //column
                        .fold(true, |s, p| s && new_pos[0]%10==p)
        }
    }

    /// Create new Cell from existing cell with new options

    pub fn copy_with_new_options(&self, new_options:&Vec<Vec<usize>>, new_is_black_listed: bool) -> Self {
        Cell {
            operation: self.operation,
            result: self.result,
            positions: self.positions.clone(),
            is_onedim: self.is_onedim,
            is_black_listed: new_is_black_listed,
            options: new_options.clone()
        }
    }

    pub fn new_from_string(cell_as_string:&str) -> Result<Self,String> {
       //parse the input line into an vec of usize containing
        // the result at index 0,
        // the (encoded) operation at index 1 and
        // the positions from index 2 till the end
        let mut positions:Vec<usize>  = cell_as_string.chars()
            //map operations to ids and insert separators
            .map(|c| match c {
                'c' => ".0.".to_string(),
                '+' => ".1.".to_string(),
                '-' => ".2.".to_string(),
                '*' => ".3.".to_string(),
                ':' => ".4.".to_string(),
                _ => c.to_string()
            })
            .collect::<String>()
            //Split Res from operation from Positions
            .split(".")
            //try to parse into number
            .map(|xs|  match xs.parse::<usize>() {
                Ok(x) => x,
                Err(_) => usize::MAX,
                })
            .collect();

        //Check if there are at least 3 entries and
        // that here where no conversion errors, i.e. no usize::MAX is in the vector
        if positions.len()>=3 &&
            positions.iter().fold(0,|max,&pos| if pos>max {pos} else {max})<usize::MAX {

            let result = positions.remove(0);
            let operation = vec!['c','+','-','*',':'][positions.remove(0)];

             Ok(Cell::new(&positions,operation,result))
        } else {
            Err(format!("Can't parse line: '{}'", cell_as_string))
        }
    }


    /// Adds the option with index option_nr to the given field
    /// no validation is done
    /// the return value indicates success (true) or failure (false),
    /// i.e. the option_nr is greater than the available options
    pub fn apply_option_to_field(&self, field: &mut Vec<usize>, option_nr: usize) -> bool {

        if option_nr<self.options.len() {
            //let mut changed:bool=false;
            self.positions.iter()
                .zip(self.options[option_nr].iter())
                .for_each(|(&p,&d)| {
                //changed = changed || field[p] != d;
                field[p]=d;
            });
            //changed
            true
        } else {false}
    }


    /// Add all possible options for the KenKen-Cell, which fulfill the mathematical restrictions
    /// check, that the option values are compliant with the 1 digit per row/column restriction
    /// This allows to add/check options position wise...
    pub fn add_options_base_kenken(&mut self, dimension:usize) -> usize{

        //create a cartesian product of the digits from 1 to the dimension of the puzzle
        //for each of the positions
        //filter out the invalid options
        //fold the option-tuple into a single usize
        self.options=(0..self.positions.len())
            .map(|_| (1..=dimension))
            .multi_cartesian_product()
            .filter(|option| self.is_valid_cell_option( option))
            //.map(|tupel| tupel.iter().fold(0, |opt, &d| 10 * opt + d))
            .collect();
        self.options.len()
    }

    /// Add all possible options for the Sudoku-Cell
    pub fn add_options_base_sudoku(&mut self, constants:&HashSet<usize>) -> usize {
        let mut data:Vec<usize>;
        let mut permutations:Vec<Vec<usize>> = Vec::new();

        data=(1..10).filter(|d| !constants.contains(d)).collect();

        heap_recursive(&mut data, |p| {
            //permutations.push(p.iter().fold(0,|s,d| s*10+d))
            permutations.push(p.to_vec())
        });

        self.options=permutations.clone();
        self.options.len()
    }

     /// Validates the options of a cell against a given field
    /// returns a new cell with all valid options and a count of the valid options

     pub fn get_valid_cell_options(&self, field: &Vec<usize>, bl: &mut BlackList) -> (usize, usize, Self) {

         //if only 1 option is left, return the current cell
         if self.options.len()==1 {
             return (1,1,self.clone());
         };

         //current options to be validated
         let mut new_options = self.options.clone();
         let mut new_black_listed = self.is_black_listed;


         //for each position
         for index in 0..self.positions.len(){
             let col = self.positions[index] % 10;
             let row = self.positions[index] - col;

             //get the black listed digits for the current position
             let mut pos_bl: HashSet<usize> = bl.get(&self.positions[index]);

             //get the existing digits in the col and row of the current position
             //add those digits to the position blacklist


             (row..row + 9).chain((col..90).step_by(10))
                 .map(|i| field[i])//change index to digit
                 .filter(|&d| d > 0)  //get existing values
                 .for_each(|d| if pos_bl.insert(d) {}); //add to positional blacklist

             //filter out all digits from the positional blacklist
             new_options = new_options.into_iter()
                 .filter(|o| !pos_bl.contains(&o[index]))
                 .collect();

         };
         //Update the blacklist if new unique values for one dimensional cells are found

         if self.is_onedim && !new_black_listed && new_options.len() > 1 {
             //println!("----\n Cell: {:?} \n bl: {:?} \n NewOpt: {:?}", self, bl, new_options);
             //get digits of first option
             let check_digits: HashSet<usize> = new_options[0].iter()
                 .map(|&d| d)
                 .collect();
             //check if any of the other options contain any digit not in the first option
             if !new_options.iter().skip(1)
                 .any(|o| o.iter()
                     .any(|d| !check_digits.contains(d))) {
                 //all available options have the same digits
                 //update the blacklist
                 bl.insert(&self.positions, &check_digits);
                 new_black_listed = true;
                 //println!("** bl after: {:?}", bl);
             }
         }
         (new_options.len(), self.positions.len(), self.copy_with_new_options(&new_options, new_black_listed))
     }

    /// Validates if candidate is a valid option for a KenKen cell
    fn is_valid_cell_option( &self, candidate:&Vec<usize>) -> bool {


        //check that no duplicates in line or column
        if !(0..candidate.len()).fold(true, |r,i| r &&
            ((0..candidate.len()).fold(0,|s,x|
                if candidate[i]==candidate[x] && self.positions[i]/10 == self.positions[x]/10  {s+1} else {s}) == 1) &&
            ((0..candidate.len()).fold(0,|s,x|
                if candidate[i]==candidate[x] && self.positions[i]%10 == self.positions[x]%10  {s+1} else {s}) == 1)) {return false}

        //checks the numeric calculation
        match self.operation {
            '+' => self.result==candidate.iter().fold(0,|s,x| s+x),
            '*' => self.result==candidate.iter().fold(1,|s,x| s*x),
            '-' => candidate.len()==2 && self.result==(candidate[1] as i32 - candidate[0] as i32).abs() as usize,
            ':' => candidate.len()==2 && ((candidate[1]== (self.result * candidate[0])) || (candidate[0]== (self.result * candidate[1]))),
            'c' => candidate.len()==1 && (candidate[0]==self.result),
            _ => false
        }

    }



}

