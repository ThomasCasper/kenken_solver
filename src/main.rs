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

use crate::kk_generate::GeneratedPuzzle;
use crate::kk_load::PuzzleAsString;
use std::env;
use std::time::Instant;

use crate::kk_puzzle::Puzzle;

mod kk_black_list;
mod kk_generate;
mod kk_group;
mod kk_load;
mod kk_puzzle;

/// The main program coordinate the steps for the solution
/// * ask user for the file name of the puzzle
/// * load the file via kk_inputs
/// * start the recursive try and error solution process
/// * print the solution
///

fn main() {
    //Retrieve filename from Args or as user input
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
    } else {
        match &args[1][0..] {
            "solve" => solve(args),
            "generate" => drop(generate(args)),
            "gen_solve" => gen_solve(args),
            _ => help(),
        }
    }
}

fn solve(args: Vec<String>) {
    if args.len() != 3 {
        help();
    } else {
        solve_kernel(PuzzleAsString::new_from_file(&args[2]).expect("Couldn't load file."));
    }
}

fn solve_kernel(puzzle_string: PuzzleAsString) {
    let now = Instant::now();

    println!("Starting to solve....\n{}", puzzle_string);

    let puzzle = Puzzle::new_from_puzzle_file(puzzle_string).expect("Init from loaded file failed");

    //solve the puzzle & print out
    let solution_option = puzzle.solve();
    if solution_option.is_some() {
        let solution = solution_option.unwrap();
        println!("Solution: \n\n{}\n", solution);
    } else {
        println!("Error! Puzzle is not solvable!");
    }
    let duration = now.elapsed().as_millis();
    println!(
        "Total Duration : {:02}:{:02}:{:02}.{:03}",
        duration / 3600000,
        duration / 60000 % 60,
        duration / 1000 % 60,
        duration % 1000
    );
}

fn generate(args: Vec<String>) -> String {
    let mut new_puzzle_string: String = String::new();
    if args.len() == 5 {
        let dimension: usize = args[2].parse().unwrap_or(100);
        let difficulty: usize = args[3].parse().unwrap_or(100);
        let operation_range: usize = args[4].parse().unwrap_or(100);
        if dimension >= 3 && dimension <= 9 && difficulty <= 3 && operation_range <= 1 {
            //println!("Generate {}x{} KenKen....\n------------------", dimension, dimension);
            let new_puzzle =
                GeneratedPuzzle::generate_kenken(dimension, difficulty, operation_range);
            new_puzzle_string = new_puzzle.to_raw_string();
            println!("{}", new_puzzle_string);
        } else {
            help();
        }
    } else {
        help();
    }

    new_puzzle_string
}

fn gen_solve(args: Vec<String>) {
    let puzzle_as_string = PuzzleAsString::new_from_raw_string(generate(args));
    if puzzle_as_string.is_ok() {
        solve_kernel(puzzle_as_string.unwrap());
    }
}

fn help() {
    println!("run mode [parameters] - starts KenKen-Solver in one of the following modes with the following parameters\n");
    println!("Modes:");
    println!("solve <path to puzzle> - prints the solution of the specified puzzle");
    println!("generate <dimension> <difficulty> <operations_range> - generates a new KenKen-puzzle with the given parameters\n");
    println!("  dimension [3-9] - the dimension/size of the KenKen");
    println!("  difficulty [0-3] - the difficulty of the KenKen 0-easy to 3-expert");
    println!("  operations_range [0,1] - the used operations in the KenKen 0-only addition, 1 - all operations");
}
