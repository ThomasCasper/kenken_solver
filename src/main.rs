//! The program KenKen_Solver solves KenKen and Sudoku puzzles
//!
//! The puzzle to solve must be specified a separate text-file with the following format
//!
//! # Kenken:
//! for more information about KenKen see [KenKen Wikipedia](https://de.wikipedia.org/wiki/Ken_Ken)
//!
//! ## File Format:
//! * first line comment
//! * second line: must start with "KenKen" (exactly)
//! * third line till end of file: the specification of the puzzle
//! * each line represents one "cell" of the KenKen.
//! Cell means the connected areas with the given result of an operation
//! * the format of each line
//! ``` [result][operation][field 1].[field 2]....[field n] ```
//! * the fields are the coordinates of the fields belonging to the cell,
//! the left upper corner is 00, the first digit is the row, the second the column
//! * the operation is one of the following
//!     * '+' - addition
//!     * '*' - multiplication
//!     * '-' - subtraction, the cell must have exactly 2 fields
//!     * ':' - division, the cell must have exactly 2 fields
//!     * 'c' - constant, the cell has exactly 1 field with a given digit (which is the result)
//! ## Examples
//! for the KenKen puzzle [Newdoku puzzle 1278350](https://newdoku.com/include/online.php?id=1278350)
//! ```
//! Newdoku.com KenKen-puzzle nr.: 1278350 with Dim 4 x 4
//! KenKen
//! 1-00.01
//! 8+02.03.12
//! 6*10.11.20
//! 2-13.23
//! 16*21.30.31
//! 6+22.32.33
//! ```
//!# Sudoku:
//! for more information about Sudoku see [Sudoku Wikipedia](https://de.wikipedia.org/wiki/Sudoku)
//!
//! ## File Format:
//! * first line comment
//! * second line: must start with "Sudoku" (exactly)
//! * third line till end of file: the specification of the puzzle
//! * each line is a row of the Sudoku puzzle,
//!     * given digits as digits,
//!     * open fields are represented as "-"
//!     * for better readability a "." might be entered between 3 position.
//!
//! #Examples
//! for a Sudokupuzzle
//! ```
//! Sudoku Expert level
//! Sudoku
//! -5-.--8.269
//! --2.-43.---
//! --9.---.---
//! --7.---.---
//! ---.-9-.-4-
//! 5-3.---.-9-
//! ---.-24.6-5
//! 6--.---.--3
//! -4-.-8-.---
//! ```
//!
use crate::kk_field::Field;
use std::time::Instant;
use std::io;

mod kk_cell;
mod kk_field;
mod kk_inputs;
mod kk_improve;

/// The main program coordinate the steps for the solution
/// * ask user for the file name of the puzzle
/// * load the file via kk_inputs
/// * start the recursice try and error solution process
/// * print the solution
///

fn main() {


    println!("Enter filename of KenKen: ");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Could not read from standard input");
    let now = Instant::now();
    let mut f = Field::new(None);
    if file_name.starts_with("Dim") {
        f.initialize_from_definition(
        &kk_inputs::definition_inline(file_name.trim())
        ).expect("Init from inline definition failed");
    } else {
        f.initialize_from_definition(
        &kk_inputs::definition_from_file(file_name.trim())
        ).expect("Init from external definition failed");
    }
    let dur=now.elapsed().as_millis();
    println!("Init Duration : {}:{}:{}.{}",dur/3600000,dur/60000 % 60,dur/1000 % 60,dur % 1000 );

    println!("Start -\n{}",f);
    //println!("{:?}", f);

    let solution = kenken_solve(1,f);
    match solution {
        Some(sol) => println!("Solution -\n{}",sol),
        None => println!("Error"),
    }
    let dur=now.elapsed().as_millis();
    println!("Total Duration : {}:{}:{}.{}",dur/3600000,dur/60000 % 60,dur/1000 % 60,dur % 1000 );

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
/// and restart the recursion, if the choosen option for the cell was wrong, choose the next option ...
///
fn kenken_solve(iteration: i32, field: Field) -> Option<Field> {

    //println!("{} -\n{}",iteration, field);
    let (count, temp_field, opt) = field.get_new_valid_field();
    //println!("{:?}{:?}{:?}", count,temp_field,opt);
    if count ==0 {
        // if count is zero recursion ends
        // if field is None there was an error
        // otherwise field contains the found solution
             return temp_field;

        };
    // new iteration

    let option = opt.unwrap();
    let mut current_option:usize = 0;


    let mut new_field: Field = temp_field.unwrap();
    while new_field.apply_option_to_field(&option,current_option) {

        current_option +=1;
        if let Some(field)=kenken_solve(iteration+1, new_field.clone()) {
            return Some(field);
        };
    };


    None

}
