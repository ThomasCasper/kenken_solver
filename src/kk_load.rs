use std::fmt;
use std::fs;

use GameType::{KenKen, Sudoku};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameType {
    KenKen,
    Sudoku,
}

#[derive(Debug, Clone, Getters)]
pub struct PuzzleAsString {
    game_type: GameType,
    description: String,
    puzzle_string: Vec<String>,
}

impl PuzzleAsString {
    pub fn new_from_raw_string(raw_puzzle_string: String) -> Result<Self, String> {
        let mut puzzle_string: Vec<String> = raw_puzzle_string
            .split('\n')
            .map(|c| c.trim().to_string())
            .collect();

        //first line of the file is the description
        let description = puzzle_string.remove(0);

        //second line is the game type
        let game_type: GameType = match &*puzzle_string.remove(0) {
            "KenKen" => KenKen,
            "Sudoku" => Sudoku,
            other_type => return Err(format!("No valid Puzzle Type '{}'", other_type)),
        };

        Ok(PuzzleAsString {
            game_type,
            description,
            puzzle_string,
        })
    }

    pub fn new_from_file(file_name: &str) -> Result<Self, String> {
        let raw_puzzle_string = match fs::read_to_string(file_name.trim()) {
            Ok(raw_puzzle) => raw_puzzle,
            Err(e) => {
                return Err(format!(
                    "Error reading file. Error message:\n{}",
                    e.to_string()
                ))
            }
        };

        PuzzleAsString::new_from_raw_string(raw_puzzle_string)
    }

    pub fn get_dimension(&self) -> Result<usize, String> {
        if self.game_type == Sudoku {
            return Ok(9);
        };

        //get all positions from the puzzle string into a vec of positions
        let mut positions_list: Vec<usize> = self
            .puzzle_string
            .join(".")
            //transform string into chars-Iterator
            .chars()
            //map operation to #. to separate result from positions
            //map line separator to ".", leave all other characters unchanged
            .map(|c| match c {
                '+' | '-' | '*' | ':' | 'c' => "#.".to_string(),
                _ => c.to_string(),
            })
            //recollect all chars into a new string
            .collect::<String>()
            //separate all positions
            .split(".")
            //parse positions into numbers, all the rest are no positions
            .map(|ps| match ps.parse::<usize>() {
                Ok(p) => p,
                Err(_) => 999,
            })
            //get rid of the non-position entries, i.e. the results (and operation)
            //the maximum possible position is 88 in 9x9 puzzle
            .filter(|&p| p <= 88)
            .collect();

        let positions_count = positions_list.len();

        positions_list.sort();
        positions_list.dedup();
        let position_count_dedup = positions_list.len();

        //the minimal 3x3 KenKen has 9 positions, the maximal 9x9 kenken 81
        if position_count_dedup == positions_count && positions_count >= 9 && positions_count <= 81
        {
            //get the maximum of the row or column of the positions
            let dim: usize = positions_list
                .iter()
                .map(|&p| if p / 10 > p % 10 { p / 10 } else { p % 10 }) //map positions to higher of row or column
                .max() //get the max, which would be the dimension -1
                .unwrap()
                + 1;

            if positions_count == dim * dim {
                return Ok(dim);
            }
        }

        Err(format!(
            "Dimension can't be determined. Field is not consistent or completely specified.\n\
                Count of positions in file vs. count without duplicates: {},{}\
                Found positions{:?}",
            positions_count, position_count_dedup, positions_list
        ))
    }
}

impl fmt::Display for PuzzleAsString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display: String = format!("{}\nType: {:?}", self.description, self.game_type);
        write!(f, "{}", display)
    }
}

#[cfg(test)]
mod kk_loader_tests {
    use super::*;

    #[test]
    fn check_load_from_file() {
        let valid_kenken_file = PuzzleAsString::new_from_file("KK-Dim4-1.txt").unwrap();
        assert_eq!(valid_kenken_file.game_type, KenKen);
        assert_eq!(
            valid_kenken_file.description,
            "Newdoku.com KenKen-puzzle nr.: 1278350 with Dim 4 x 4"
        );
        //assert_eq!(valid_kenken_file.puzzle_string, "1-00.01|8+02.03.12|6*10.11.20|2-13.23|16*21.30.31|6+22.32.33");

        let valid_sudoku_file = PuzzleAsString::new_from_file("S-1.txt").unwrap();
        assert_eq!(valid_sudoku_file.game_type, Sudoku);
        assert_eq!(
            valid_sudoku_file.description,
            "Sudoku Expert - https://sudoku.com/de/experte/"
        );
        //assert_eq!(valid_sudoku_file.puzzle_string, "-5-.--8.269|--2.-43.---|--9.---.---|--7.---.---|---.-9-.-4-|5-3.---.-9-|---.-24.6-5|6--.---.--3|-4-.-8-.---");

        let invalid_file = PuzzleAsString::new_from_file("test_fail");
        assert_eq!(invalid_file.is_err(), true);
        assert_eq!(
            invalid_file.unwrap_err(),
            "Error reading file. Error message:\nNo such file or directory (os error 2)"
        );
    }

    #[test]
    fn check_get_dimension() {
        let kenken_1 = PuzzleAsString::new_from_file("KK-Dim4-1.txt").unwrap();
        assert_eq!(kenken_1.get_dimension(), Ok(4));

        let kenken_2 = PuzzleAsString::new_from_file("KK-Dim9-1.txt").unwrap();
        assert_eq!(kenken_2.get_dimension(), Ok(9));

        let sudoku_1 = PuzzleAsString::new_from_file("S-1.txt").unwrap();
        assert_eq!(sudoku_1.get_dimension(), Ok(9));
    }
}
