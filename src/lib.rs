#[macro_use]
extern crate derive_getters;

use crate::kk_load::PuzzleAsString;
use crate::kk_puzzle::Puzzle;

pub mod kk_block_list;
pub mod kk_generate;
pub mod kk_group;
pub mod kk_load;
pub mod kk_puzzle;


pub fn solve(puzzle_string:PuzzleAsString)-> Option<Vec<usize>> {
    let puzzle = Puzzle::new_from_puzzle_file(puzzle_string).expect("Init from loaded file failed");
    if let Some(solution) = puzzle.solve() {
        return Some(solution.solution().clone());
    } else {
        return None;
    }
}