use std::collections::HashSet;

use permutohedron::heap_recursive;

use crate::kk_improve::BlackList;

#[derive(Debug,Clone)]
pub struct Cell {
    res: u32,
    ops: char,
    pos:Vec<usize>,
    options: Vec<u32>,
    pub is_onedim: bool,
    pub is_black_listed: bool
}


/*#[derive(Debug,Clone)]
pub struct ReducePosByDigits {
    pos: HashSet<usize>,
    digits: HashSet<u32>
}

impl ReducePosByDigits {

    /// Creates new ReducePosByDigit_Struct
    /// with new_digits as digits
    /// mode: True - row, false - col
    /// line: the row or col of all cells
    /// line_pos: the position of the "unique digits", hence the pos field is the reverse
    pub fn new(new_digits: HashSet<u32>, modus:bool, line:usize, line_pos: HashSet<usize>) -> Self {
        let mut new_pos = HashSet::<usize>::new();
        for i in 0..9 {
            if !line_pos.contains(&i) {
                if modus { new_pos.insert(10 * line + i) }
                else {new_pos.insert(10*i+line)};
            };
        };

        ReducePosByDigits {
            pos: new_pos,
            digits : new_digits,
        }
    }
}*/


pub fn get_digits(val:u32) -> Vec<u32>{
    let mut v=Vec::<u32>::new();
    let mut j:u32=val;
    loop {
        v.push(j % 10);
        j /= 10;
        if j==0 {break}
    };
    v.reverse();
    v
}

impl Cell {
    /// Create new Cell with position, ops and res
    /// There are no options attached
    /// use add_options_base for the initial options based on result and ops
    /// or use add_option for direct option attachment
    pub fn new(new_pos: &Vec<usize>, new_ops: char, new_res: u32) -> Self {

        Cell {
            ops: new_ops,
            res: new_res,
            pos: new_pos.clone(),
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

    pub fn new_with_options(old_cell: &Self, new_options:&Vec<u32>, new_is_black_listed: bool) -> Self {
        Cell {
            ops: old_cell.ops,
            res: old_cell.res,
            pos: old_cell.pos.clone(),
            is_onedim: old_cell.is_onedim,
            is_black_listed: new_is_black_listed,
            options: new_options.clone()
        }
    }

    /// If a constant is found, just add the constant to the field
    /// for constants there is no "try-and-error" required
    pub fn apply_const(&self, field: &mut Vec<u32>, check_field: &mut Vec<u32>) -> Result<bool,String> {
        if self.ops =='c' {
            if self.pos.len() != 1 {return Err(format!("Constant [{}] with ambiguous position(s): {:?}",self.res,self.pos))};
            field[self.pos[0]] = self.res;
            check_field [self.pos[0]] += 1;
            Ok(true)
        } else {Ok(false)}
    }

    /// Adds the option with index option_nr to the given field
    /// no validation is done
    /// the return value indicates success (true) or failure (false),
    /// i.e. the option_nr is greater than the available options
    pub fn apply_option_to_field(&self, field: &mut Vec<u32>, option_nr: usize) -> bool {

        if option_nr<self.options.len() {
            let digits = get_digits(self.options[option_nr]);
            self.pos.iter().zip(digits.iter()).for_each(|(&p,&d)| field[p]=d);
            true
        } else {false}
    }

    /// Mark all positions fo the Cell in the game field, to check completeness of KenKen puzzle
    pub fn mark_positions(&self, field: &mut Vec<u32>) {
        //self.pos.iter().map(|&x | field[x]  +=1);
        for p in &self.pos {
            field[*p] += 1;
        }
    }

    /// Add all possible options for the KenKen-Cell, which fulfill the mathematical restrictions
    /// check, that the option values are compliant with the 1 digit per row/column restriction
    /// This allows to add/check options position wise...
    pub fn add_options_base_kenken(&mut self, kk_size:u32) -> u32{
        let start:u32 = u32::pow(10, self.pos.len() as u32-1);
        let ops=self.ops;
        let res= self.res;
        let pos:Vec<usize> = self.pos.clone();
        for o in
            (start..10*start)
                .filter(|&x| Cell::validate_kenken_candidate(&pos, kk_size, ops,res,x)) {
            self.options.push(o)
        }

        self.options.len() as u32
    }

    /// Add all possible options for the Sudoku-Cell
    pub fn add_options_base_sudoku(&mut self, constants:&HashSet<u32>) -> u32 {
        let mut data:Vec<u32>;
        let mut permutations:Vec<u32> = Vec::new();

        data=(1..10).filter(|d| !constants.contains(d)).collect();

        heap_recursive(&mut data, |p| {
            permutations.push(p.iter().fold(0,|s,d| s*10+d))
        });

        self.options=permutations.clone();
        self.options.len() as u32
    }

     /// Validates the options of a cell against a given field
    /// returns a new cell with all valid options and a count of the valid options

     pub fn get_valid_cell_options(&self, field: &Vec<u32>, bl: &mut BlackList) -> (usize, Self) {

         //current options to be validated
         let mut new_options = self.options.clone();
         let mut new_black_listed = self.is_black_listed;

         //position of digit of current position
         let mut pod: u32 = u32::pow(10, self.pos.len() as u32 - 1);
         //for each position
         for p in &self.pos {
             let col = p % 10;
             let row = p - col;

             //get the black listed digits for the current position
             let mut pos_bl: HashSet<u32> = bl.get(p);

             //get the existing digits in the col and row of the current position
             //add those digits to the position blacklist


             (row..row + 9).chain((col..90).step_by(10))
                 .map(|i| field[i])//change index to digit
                 .filter(|&d| d > 0)  //get existing values
                 .for_each(|d| if pos_bl.insert(d) {}); //add to positional blacklist

             //filter out all digits from the positional blacklist
             new_options = new_options.into_iter()
                 .filter(|&o| !pos_bl.contains(&((o / pod) % 10)))
                 .collect();

             pod /= 10;
         };
         //Update the blacklist if new unique values for one dimensional cells are found

         if self.is_onedim && !new_black_listed && new_options.len() > 1 {
             //println!("----\n Cell: {:?} \n bl: {:?} \n NewOpt: {:?}", self, bl, new_options);
             //get digits of first option
             let check_digits: HashSet<u32> = get_digits(new_options[0]).into_iter().collect();
             //check if any of the other options contain any digit not in the first option
             if !new_options.iter().skip(1)
                 .any(|&o| get_digits(o).iter()
                     .any(|d| !check_digits.contains(d))) {
                 //all available options have the same digits
                 //update the blacklist
                 bl.insert(&self.pos, &check_digits);
                 new_black_listed = true;
                 //println!("** bl after: {:?}", bl);
             }
         }
         (new_options.len(), Cell::new_with_options(self, &new_options, new_black_listed))
     }



    /// Validates if candidate is a valid option for a KenKen cell
    fn validate_kenken_candidate(pos: &Vec<usize>, kk_size: u32, op:char, res:u32, candidate:u32) -> bool {

        //decompose candidate into single digits
        let v = get_digits(candidate);

        //check if candidate includes zeros or digits greater than the kk_size
        if v.iter().fold(0, |s,&x| if x==0 || x>kk_size {s+1} else {s}) >0 {
            return false;
        }

        //check that no duplicates in line or column
        if !(0..v.len()).fold(true, |r,i| r &&
                ((0..v.len()).fold(0,|s,x|
                    if v[i]==v[x] && pos[i]/10 == pos[x]/10  {s+1} else {s}) == 1) &&
                ((0..v.len()).fold(0,|s,x|
                    if v[i]==v[x] && pos[i]%10 == pos[x]%10  {s+1} else {s}) == 1)) {return false}

        //checks the numeric calculation
        match op {
            '+' => res==v.iter().fold(0,|s,x| s+x),
            '*' => res==v.iter().fold(1,|s,x| s*x),
            '-' => v.len()==2 && res==(v[1] as i32 - v[0] as i32).abs() as u32,
            ':' => v.len()==2 && ((v[1]== (res* v[0])) || (v[0]== (res* v[1]))),
            _ => false
            }


    }

/*    /// check_cell_on_unique_digits_per_line checks if all positions of the cell
    /// are in the same row or column and if the valid options contain exactly the same
    /// number of different digits
    /// (e.g. a Cell with "8-" and 2 positions only has 1-9 and 9-1 as valid options)
    /// if unique digits per line are found the digits and the remaining positions in
    /// the same line are returned
    pub fn check_cell_on_unique_digits_per_line(&self)->Option<ReducePosByDigits>{
        let mut col_hash=HashSet::<usize>::new();
        let mut row_hash=HashSet::<usize>::new();
        //get different rows/cols from positions
        for pos in &self.pos {
            col_hash.insert(pos % 10);
            row_hash.insert(pos / 10);
        };
        //if multiple rows/columns -> return
        if col_hash.len()>1 && row_hash.len()>1 { return None};
        //Values are in the same line/column
        let line: usize;
        let line_hash: HashSet<usize>;
        let modus:bool;
        if col_hash.len()==1 {
            line = self.pos[0] % 10;
            line_hash = row_hash;
            modus = false;
        } else {
            line = self.pos[0] / 10;
            line_hash = col_hash;
            modus = true;
        };
        let dc = line_hash.len();
        let mut digit_hash = HashSet::<u32>::new();
        for op in &self.options {
            //get unique digits from option
            let mut j:u32=*op;
            loop {
                digit_hash.insert(j % 10);
                j /= 10;
                if j==0 {break}
            };
            //if there are more different digits than positions -> return
            if digit_hash.len()>dc {return None}
        }
        //we have found unique line values

        Some(ReducePosByDigits::new(digit_hash,modus,line,line_hash))

    }

    /// clean_unique_digits_from_line reduces the unique digits from the valid options
    /// if the cell contains one ore more positions in ReducePosByPosition
    /// if a cleaning took place the return value is true
    pub fn clean_unique_digits_from_line(&mut self, rpv: &Vec<ReducePosByDigits>) -> bool {

        let mut changed:bool=false;

        for (i,pos) in (&self.pos).iter().enumerate() {
            for rp in rpv {
                if rp.pos.contains(pos) {
                    let mut new_options: Vec<u32> = Vec::new();
                    let mut pos_changed: bool = false;
                    //println!("found pos {} at {} in {:?}", pos, i, &self);
                    //Cell contains a relevant position
                    for &op in &self.options {
                        if rp.digits.contains(&get_digits(op)[i]) {
                            //Option with unique value was found and will be removed
                            pos_changed = true;
                        } else {
                            new_options.push(op);
                        }
                    }
                    if pos_changed {
                        //println!("before change {:?}",&self);
                        self.options = new_options;
                        changed = true;
                        //println!("after change {:?}",&self);
                    }
                }
            }
        }

        changed
    }*/

}

