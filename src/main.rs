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
//! for a Sudoku puzzle
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

#[macro_use]
extern crate derive_getters;

use crate::kk_field::Field;
use std::time::Instant;
use std::io;
use std::env;

mod kk_cell;
mod kk_field;
mod kk_improve;
mod kk_load;

/// The main program coordinate the steps for the solution
/// * ask user for the file name of the puzzle
/// * load the file via kk_inputs
/// * start the recursive try and error solution process
/// * print the solution
///

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_name = String::new();
    if args.len() == 1 {
        println!("Enter filename of puzzle: ");
        io::stdin().read_line(&mut file_name).expect("Could not read from standard input");
    } else {
        file_name = args[1].clone();
    }

    let now = Instant::now();
    let puzzle_string = kk_load::PuzzleAsString::new_from_file(&file_name)
        .expect("Couldn't load file.");

    println!("Starting to solve....\n{}", puzzle_string);

    let mut field = Field::new();
    field.initialize_from_puzzle_file(puzzle_string)
        .expect("Init from loaded file failed");

    let solution = field.solve();
    match solution {
        Some(sol) => println!("Solution: \n\n{}\n", sol),
        None => println!("Error! Puzzle is not solvable!"),
    }
    let duration = now.elapsed().as_millis();
    println!("Total Duration : {:02}:{:02}:{:02}.{:03}", duration / 3600000, duration / 60000 % 60, duration / 1000 % 60, duration % 1000);
}


