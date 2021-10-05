//use crate::kk_cell::{Cell, ReducePosByDigits};
use crate::kk_cell::{Cell};
use std::fmt;
use crate::kk_field::GameType::{KenKen, Sudoku};
use std::collections::HashSet;
use crate::kk_improve::BlackList;
use std::collections::HashMap;


#[derive(Debug,Clone)]
pub struct Field {
    game_type: GameType,
    dim: usize,
    field:Vec<u32>,
    bl:BlackList,
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
                bl:of.bl.clone(),
                cells: Vec::new(),
            }

        } else {

            Field {
                game_type: KenKen,
                dim: 0,
                field: vec![0; 100],
                bl: BlackList::new(),
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

                //if !cell.apply_const(&mut self.field, &mut check_field)? {
                    self.cells.push(cell);
                //};

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

        //initialize blacklist and apply first unique digits
        let (_cnt, o_field,c)= self.get_new_valid_field();
        //println!("Init: {} - {:?} - {:?}",cnt,o_field,c);
        if let Some(of)=o_field {
            self.field = of.field.clone();
            self.bl = of.bl.clone();
            self.cells = of.cells.clone();
            self.cells.push(c.unwrap());  //add best cell to cells
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
    /// if count is 0, and a field is returned: The Kenken was solved and the returned field is the solution
    /// if count is 0 and the field is None, there where no valid options left and the try was an error

    pub fn get_new_valid_field(&self) -> (usize, Option<Self>, Option<Cell>) {
        let mut new_field = Field::new(Some(&self));
        let mut new_cells = self.cells.clone();
        let mut ind:usize = 0;
        let mut cells_w_options:usize = 0;
        let mut ind_min:usize=0;
        let mut min_cells:usize=1000;
        //println!("New validation: {}", new_cells.len());
        while ind < new_cells.len() {
            //println!("{} - {}",ind, new_cells.len());
            let (cell_cnt, valid_cell) = new_cells.remove(ind)
                .get_valid_cell_options(&new_field.field,&mut new_field.bl);

            match cell_cnt {
                // no valid options left => Error and next try
                0 => {
                    //println!("Cell with count 0: {} - {:?}",ind,valid_cell);
                    //println!("New field with cnt 0: {:?}", new_field);
                    return (0, None, None);
                },
                // only 1 option left => Add option (first) to field and restart update
                1 => {


                    //only "reset" if change happened
                    if valid_cell.apply_option_to_field(&mut new_field.field, 0) {
                        new_cells.insert(ind,valid_cell);
                        min_cells = 1000;
                        ind = 0;
                        cells_w_options=0;
                    } else {
                      new_cells.insert(ind,valid_cell);
                        ind += 1;
                    };

                },
                // more than 1 option left, add cell back to list and move to next cell
                // if number of valid options is the new min, than save the index
                c => {
                    new_cells.insert(ind,valid_cell);
                    cells_w_options+=1;
                    if c<min_cells {
                        min_cells=c;
                        ind_min=ind;
                    };
                    ind+=1;
                }
            }
        }

        //if new_cells.len()>0 {
        if cells_w_options>0 {
            let best_option= new_cells.remove(ind_min);
            new_field.cells = new_cells.clone();
            (cells_w_options, Some(new_field),Some(best_option))
        }
        else {
            new_field.cells = new_cells.clone();
            (0,Some(new_field),None)
        }

    }

    pub fn apply_option_to_field(&mut self, cell: &Cell, option: usize) -> bool {

        if cell.apply_option_to_field(& mut self.field, option) {
           self.cells.push(Cell::new_with_option_nr(cell,option));
            true
        } else {
            false
        }
    }
}




impl fmt::Display for Field {
    // This trait requires `fmt` with this exact signature.
/*    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    }*/
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

       let cross_marks: HashMap<u32, char> = [
            ( 0, '\u{253C}'),
            ( 5, '\u{253F}'),
            (10, '\u{2542}'),
            ( 9, '\u{2543}'),
            (12, '\u{2544}'),
            ( 3, '\u{2545}'),
            ( 6, '\u{2546}'),
            (13, '\u{2547}'),
            ( 7, '\u{2548}'),
            (11, '\u{2549}'),
            (14, '\u{254A}'),
            (15, '\u{254B}')
            ].iter().cloned().collect();

        let d:usize = self.dim;

        //Extract groups from Cells
        let mut cell_field:Vec<u32>=vec![0;100];
        //for Kenken
        if self.game_type == KenKen {
            let mut cell_id: u32 = 0;
            for c in &self.cells {
                cell_id += 1;
                c.mark_cell_positions(&mut cell_field, cell_id);
            };
        } else {
            for row in 0..9 {
                for col in 0.. 9 {
                    cell_field[30*(row / 3)+3*(row % 3) + 10*(col /3) + (col % 3)] = row as u32 +1;
                }
            }
        };
        //println!("Len of cells: {}", self.cells.len());
        //println!("{:?}",cell_field);

        //build up display
        let mut display_field:Vec<char>=vec![' ';400];
        // fill values and lines between cells
        for row in 0..d {
            //insert top border above values
            display_field[2*row+1]='\u{2501}';
            //insert left border left of values
            display_field[40*row+20]='\u{2503}';
            //Insert Linebreaks at end of each line
            display_field[40*row+19]='\n';
            display_field[40*row+39]='\n';

            for col in 0..d {
                //insert value of cell
                if self.field[10 * row + col] > 0 {
                    display_field[40 * row + 2 * col + 21] = char::from_digit(self.field[10 * row + col], 10).unwrap();
                }
                //insert line right of cell
                if cell_field[10 * row + col] == cell_field[10 * row + col + 1] {
                    display_field[40 * row + 2 * col + 22] = '\u{2502}';
                } else {
                    display_field[40 * row + 2 * col + 22] = '\u{2503}';
                };
                //insert line at bottom of cell
                if cell_field[10 * row + col] == cell_field[10 * row + col + 10] {
                    display_field[40 * row + 2 * col + 41] = '\u{2500}';
                } else {
                    display_field[40 * row + 2 * col + 41] = '\u{2501}';
                };
            };
        }


        //add borders & cross points
        for row in 0..d-1 {
            //add top border between values
            if display_field[2*row+22]=='\u{2502}' {
               display_field[2*row+2]='\u{252F}';
            } else {
               display_field[2*row+2]='\u{2533}';
            }
            //add bottom border between values
            if display_field[40*d+2*row-18]=='\u{2502}' {
               display_field[40*d+2*row+2]='\u{2537}';
            } else {
               display_field[40*d+2*row+2]='\u{253B}';
            }
            //add left border between values
            if display_field[40*row+41]=='\u{2500}' {
               display_field[40*row+40]='\u{2520}';
            } else {
               display_field[40*row+40]='\u{2523}';
            }
            //add right border between values
            if display_field[40*row+2*d+39]=='\u{2500}' {
               display_field[40*row+2*d+40]='\u{2528}';
            } else {
               display_field[40*row+2*d+40]='\u{252B}';
            }
            for col in 0..d-1 {
                display_field[40*row+2*col+42]=*cross_marks.get(
                    &((display_field[40*row+2*col+22] as u32 - 0x2502) * 8 +
                    (display_field[40*row+2*col+43] as u32 - 0x2500) * 4 +
                    (display_field[40*row+2*col+62] as u32 - 0x2502) * 2 +
                    (display_field[40*row+2*col+41] as u32 - 0x2500))
                ).unwrap();

            }

        };

        //add corners
        //upper left
        display_field[0]='\u{250F}';
        //upper right
        display_field[2*d]='\u{2513}';
        //lower left
        display_field[40*d]='\u{2517}';
        //lower right
        display_field[42*d]='\u{251B}';

        //Print field
        //remove thin lines for better readability
        let my_display : String = display_field.iter()
            .map(|c| if (*c=='\u{2500}') || (*c=='\u{2502}') || (*c=='\u{253C}') {&' '} else {c})
            .cloned().collect();

        write!(f, "{}", my_display)

    }
}