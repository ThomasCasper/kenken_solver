use crate::kk_cell::{Cell, ReducePosByDigits};
use std::fmt;
use crate::kk_field::GameType::{KenKen, Sudoku};
use std::collections::HashSet;

#[derive(Debug,Clone)]
pub struct Field {
    game_type: GameType,
    dim: u32,
    field:Vec<u32>,
    cells:Vec<Cell>
}

#[derive(Debug, PartialEq)]
enum ParseMod {
    Result,
    Position
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum GameType {
    KenKen,
    Sudoku
}

impl Field {
    pub fn new(old_field: Option<&Field>) -> Self {

        if let Some(of) = old_field {
            Field {
                game_type: of.game_type,
                dim: of.dim,
                field: of.field.clone(),
                cells: Vec::new(),
            }

        } else {

            Field {
                game_type: KenKen,
                dim: 0,
                field: vec![0; 100],
                cells: Vec::new(),
            }
        }
    }

    pub fn initialize_from_definition(&mut self, definition: &Vec<String>) -> Result<&str, String> {
        let mut def:Vec<String>=definition.clone();
        let puzzle_typ=def.remove(0);
        println!("Type: {}", puzzle_typ);
        if puzzle_typ.starts_with("Sudoku") {
            self.game_type = Sudoku;
            self.initialize_sudoku_from_definition(&def)?;
            Ok("ok")
        } else if puzzle_typ.starts_with("KenKen") {
            self.game_type = KenKen;
            self.initialize_kenken_from_definition(&def)?;
            Ok("ok")
        } else {
            Err(format!("No valid input file! - Can't detect type of puzzle"))
        }
    }
    fn initialize_sudoku_from_definition(&mut self, definition: &Vec<String>) -> Result<&str, String> {
        //Dimension of Sudoku is always 9
        self.dim = 9;
        //derive field from input strings
        //remember for addressing each row contains 10 digits, hence the join with a 0
        //the length of the field must be 89 = 8*10+9
        self.field = definition.join("0")
            .replace(".","")
            .replace("-","0")
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if self.field.len() != 89 {
            println!("Field: {} \n {:?}", self.field.len(), self.field);
            return Err(format!("No valid Sudoku found."))};

        for quadrant in 0..9 {
            //println!("Quadrant: {}", quadrant);
            let mut constants:HashSet<u32>=HashSet::new();
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
                //println!("{:?}",cell);
                //println!("{:?}",constants);
                if cell.add_options_base_sudoku(&constants) == 0 {
                    return Err(format!("Quadrant with no valid options found {}", quadrant));
                }
                self.cells.push(cell);
            }
        }

        Ok("ok")
    }

    fn initialize_kenken_from_definition(&mut self, definition: &Vec<String>) -> Result<&str, String> {
        //Build cells for Kenken, apply constants and drop constants from list
        let mut check_field = vec![0;100];
        for l in definition {
            if let Some(cell) = Field::line_to_cell(l) {

                if !cell.apply_const(&mut self.field, &mut check_field)? {
                    self.cells.push(cell);
                };

            } else {
                return Err(format!("Can't convert line [{}] into cell!",l));
            }

        }

        //check completeness of KenKen and get Dimension
        for cell in &self.cells {
            cell.mark_positions(&mut check_field);
        }
        //println!("check_{:?}", check_field);

        // cc.0 - position index,
        // cc.1 - count of 1s,
        // cc.2 - position of last 1,
        // cc.3 - position if last value other than 0 or 1
        let cc =
            check_field.iter().fold((0, 0 ,0,100),
                                    |c,x| match x {
                                        0 => (c.0+1, c.1,c.2,c.3),
                                        1 => (c.0+1, c.1+1,c.0,c.3),
                                        _ => (c.0+1, c.1,c.2,c.0)
                                    });
        // Dimension is position of last 1 (due to 0 based indexing position is +1)
        // Number of 1s must be exactly dimension^2
        // No other values besides 0 and 1 is allowed
        if (cc.2/10 != cc.2 % 10) || (cc.1 != (cc.2/10+1)*(cc.2/10+1)) || (cc.3 < 100){
            return Err(format!("Cells in given Kenken doesn't cover field - {:?}",cc));
        }
        self.dim = cc.2/10+1;

        //Add options to Cells
        for cell in &mut self.cells {
            if cell.add_options_base_kenken(self.dim) == 0 {
                return Err(format!("Cell has no valid option - {:?}",cell));
            }
        }

        //Reduce options by eliminating unique line values
        let mut changed: bool = true;

        while changed {
            changed = false;
            let mut rp:Vec<ReducePosByDigits>=Vec::new();

            for cell in &self.cells {
                if let Some(frp)=cell.check_cell_on_unique_digits_per_line(){
                    rp.push(frp);
                }
            };
            if rp.len()>0 {
                for cell in &mut self.cells {
                    let c = cell.clean_unique_digits_from_line(&rp);
                    changed = c || changed;
                }
            }
        }

        Ok("ok")
    }

    fn line_to_cell(line:&str) -> Option<Cell> {
        let mut modus:ParseMod = ParseMod::Result;
        let mut res: u32=0;
        let mut ops:char=' ';
        let mut np:usize=0;
        let mut pos:Vec<usize>=Vec::new();

        for c in line.chars() {
            match (&modus,c) {
                (ParseMod::Result,'+') => {ops='+'; modus=ParseMod::Position},
                (ParseMod::Result,'-') => {ops='-'; modus=ParseMod::Position},
                (ParseMod::Result,'*') => {ops='*'; modus=ParseMod::Position},
                (ParseMod::Result,':') => {ops=':'; modus=ParseMod::Position},
                (ParseMod::Result,'c') => {ops='c'; modus=ParseMod::Position},
                (ParseMod::Result, _) => if let Some(d) = c.to_digit(10) { res=10*res +d as u32} else {return None},
                (ParseMod::Position,'.') => {pos.push(np); np=0},
                (ParseMod::Position,_) => if let Some(d) = c.to_digit(10) { np=10*np +d as usize} else {return None},
            }
        }
        if modus==ParseMod::Result {return None};
        pos.push(np);

        Some(Cell::new(&pos,ops,res))

    }



    /// Validates the cells of a field against a given field
    /// adds all options with no choices left, i.e. only one option was available
    /// returns a new field with all undecided cell with the open options,
    /// a count of the undecided cells and the next Cell to get "tried"
    /// the cell for the net try has the shortest possible length of open options
    /// the cell for the next try is not part of the returned new field
    /// if the count is 0, no Cell will be returned
    /// if count is 0, and a field os returned: The Kenken was solved and the returned field is the solution
    /// if count is 0 and the field is None, there where no valid options left and the try was an error

    pub fn get_new_valid_field(&self) -> (usize, Option<Self>, Option<Cell>) {
        let mut new_field = Field::new(Some(&self));
        let mut changed: bool = true;
        let mut old_cells = self.cells.clone();
        let mut min_options:usize;
        let mut best_option: Option<Cell>=None;

        //loop through all cells and reduce options and complete field
        //Returns
        // * cleaned list of Cells in old_cells
        // * best cell with minimal open options
        // returns if unfulfillable options are encountered

        while changed {
            changed=false;
            min_options=1000;
            best_option = None;
            let mut new_cells: Vec<Cell>= Vec::new();
            for cell in old_cells {
                //println!("before: {}",&new_field);
                let rc = cell.get_valid_cell_options(&new_field.field);
                //println!("after: {}",&new_field);
                //println!("rc: {:?}", rc);
                match rc.0 {
                    // no valid options left => Error and next try
                    0 => return(0,None,None),
                    // only 1 option left => Add option (first) to field and set changed to true
                    1 => {
                        rc.1.apply_option_to_field(&mut new_field.field,0);
                        changed=true
                        },
                    // greater than 1
                    // if open options less than current min save for min options
                    // else add cell to new_cells
                    c => {

                        if c < min_options {
                            if let Some(bo)= best_option { new_cells.push(bo)};
                            best_option=Some(rc.1.clone());
                            min_options=c;
                        } else {
                            new_cells.push(rc.1);
                        }
                    }
                }
            }
            old_cells = new_cells.clone();
            if changed && min_options<1000 {
                // add best_option to list for next run
                old_cells.push(best_option.clone().unwrap());
            }

        }

        new_field.cells = old_cells.clone();

        (old_cells.len(), Some(new_field),best_option)

    }

    pub fn apply_option_to_field(&mut self, cell: &Cell, option: usize) -> bool {
        cell.apply_option_to_field(& mut self.field, option)
    }
}




impl fmt::Display for Field {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut my_display = String::new();
        let d = self.dim;
        self.field.iter().fold(0, |l, &c|
            {
                if (l % 10) < d && (l / 10) < d {
                    my_display.push_str(&c.to_string())
                } else if (l % 10) == d && (l / 10) < d {
                    my_display.push('\n');
                }
                l + 1
            }
         );

        write!(f, "{}", my_display)
    }
}