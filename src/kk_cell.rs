use std::collections::HashSet;
use permutohedron::heap_recursive;

#[derive(Debug,Clone)]
pub struct Cell {
    res: u32,
    ops: char,
    pos:Vec<usize>,
    options: Vec<u32>,
}


#[derive(Debug,Clone)]
pub struct ReducePosByDigits {
    pos: HashSet<usize>,
    digits: HashSet<u32>
}

impl ReducePosByDigits {

    /// Creates new ReducePosByDigit_Struct
    /// with new_digits as digits
    /// mode: True - row, false - col
    /// line: the row or call of all cells
    /// line_pos: the position of the "unique digits", hence the pos field ist the reverse
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
}


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
            options: Vec::new()
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
            let d = get_digits(self.options[option_nr]);
            let mut i=0;
            for p in &self.pos {
                field[*p] = d[i];
                i += 1;
            }
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

    /// Add a new option manually and return count of options in Cell
    pub fn add_option_to_cell(&mut self, opt: u32) -> usize {
        self.options.push(opt);
        self.options.len()
    }

    /// Validates the options of a cell against a given field
    /// returns a new cell with all valid options and a count of the valid options

    pub fn get_valid_cell_options(&self, field: &Vec<u32>) -> (usize, Self) {
        let mut new_options=Cell::new(&self.pos, self.ops, self.res);
        let mut count: usize = 0;

        for o in &self.options {
            let mut valid: bool = true;
            let d = get_digits(*o);
            for i in 0..self.pos.len() {
                let row = self.pos[i]/10;
                let col=self.pos[i] % 10;
                valid = valid &&
                    field.iter().fold((0,0), |ind,&x|
                        if x == d[i] && ((ind.0/10) == row || (ind.0 % 10) == col) {
                            (ind.0+1,ind.1+1)
                        } else {
                            (ind.0+1,ind.1)
                        }
                   ).1 == 0;
            }
            //Option is still valid
            if valid {
                count=new_options.add_option_to_cell(*o);
            };
        }

        (count, new_options)
    }




    /// Validates if candidate is a valid option for a KenKen cell
    fn validate_kenken_candidate(pos: &Vec<usize>, kk_size: u32, op:char, res:u32, candidate:u32) -> bool {

        //decompose candidate into single digits
        let mut v = get_digits(candidate);

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


        //sorts the first 2 Elements - candidate must be greater than 9!
        if v[0]>v[1] {
            v[1] = v[0]+v[1];
            v[0] = v[1]-v[0];
            v[1] = v[1]-v[0];
        };

        //checks the numeric calculation
        match op {
            '+' => res==v.iter().fold(0,|s,x| s+x),
            '*' => res==v.iter().fold(1,|s,x| s*x),
            '-' => v.len()==2 && res==(v[1]-v[0]),
            ':' => v.len()==2 && res==(v[1]/v[0]) && (0 == v[1] % v[0]),
            _ => false
            }


    }

    /// check_cell_on_unique_digits_per_line checks if all positions of the cell
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
    }

}

