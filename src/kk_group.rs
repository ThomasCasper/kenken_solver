//! kk_groups is part of kenken_solve and provides the implementation of a group
//!
//! A group is the combination of single positions on the kenken field which fulfill
//! a given mathematical operation.
//! The Kenken puzzle consists of a set of groups. This set outlays the n x n kenken field
//!
//! For Sudoku, the groups are the open 3x3 sub-field, which also need to hold disjunctive digits.
//! (the given constants are not part of these groups.
//!
use std::collections::HashSet;

use itertools::Itertools;
use permutohedron::heap_recursive;

use crate::kk_black_list::BlackList;

/// Struct group describes a single group
/// A group consists of
/// * a result of the mathematical operation
/// * the mathematical operation ('+', '-', '*', ':' or 'c' for constants)
/// * the (vector of) single positions within the kenken puzzle belonging to the group
/// * the (vector of) the possible options (solutions) for the group full filling the operation
///   (the solution contains exactly one option)
/// * a marker, if the group is one dimensional, i.e. all positions are in exactly one row or column
/// * a marker, if a the digits of the (one dimensional) group are already added to the blacklist
///   of the puzzle. Digits are added to the blacklist, if the group is one dimensional and
///   all still valid options consist of the same m digits (in different order),
///   where m is the number of positions of the group.
///   (in this case the digits of this group are covered by this group and cannot be part of
///    valid options in other groups on the same row or column, depending on the direction of the
///    group.
#[derive(Debug, Clone, Getters)]
pub struct Group {
    result: usize,
    operation: char,
    positions: Vec<usize>,
    options: Vec<Vec<usize>>,
    is_one_dimensional: bool,
    is_already_in_black_list: bool,
}

impl Group {
    /// Creates a new group for a Sudoku puzzle
    /// Input:
    ///  * positions - the unset/looked for positions in a 3x3 subfield
    ///  * constants - the given constants in the same 3x3 subfield
    ///
    /// Returns: a result of
    ///  * a new group, if valid options are available or
    ///  * an error String otherwise
    ///
    /// the valid options are all permutations of the digits from 1 to 9
    /// which are not part of the given constants
    ///
    pub fn new_sudoku(positions: &Vec<usize>, constants: &HashSet<usize>) -> Result<Self, String> {
        let mut data: Vec<usize>;
        let mut options: Vec<Vec<usize>> = Vec::new();

        data = (1..10).filter(|d| !constants.contains(d)).collect();

        heap_recursive(&mut data, |p| {
            //permutations.push(p.iter().fold(0,|s,d| s*10+d))
            options.push(p.to_vec())
        });

        //result and operation is not relevant for Sudoku, hence the group
        //is constructed with dummy values
        //since the possibility of one dimensionality and hence useful blacklisting
        //is pretty useless, all groups are set to non-one-dimensional to prevent
        //blacklisting
        let new_group = Group {
            operation: 's',
            result: 0,
            options,
            is_already_in_black_list: true,
            is_one_dimensional: false,
            positions: positions.clone(),
        };

        if new_group.options.len() > 0 {
            Ok(new_group)
        } else {
            Err(format!("Can't find valid options for Sudoku group!"))
        }
    }

    /// Creates a new group for a Kenken puzzle
    /// Input:
    ///  * dimension - the dimension of the KenKen puzzle
    ///  * group_as_string - a string describing the group. The string is loaded from the input.
    ///
    /// Returns: a result of
    ///  * a new group, if string could be parsed and valid options are available or
    ///  * an error String otherwise
    ///
    /// First the group_as_string is parsed into positions, result and operation
    /// Afterwards the valid options are added
    /// as all combinations of digits 1 to dimension of the puzzle and
    /// fulfilling the given operation with the given result.

    pub fn new_kenken(dimension: usize, group_as_string: &str) -> Result<Self, String> {
        //parse the input line into an vec of usize containing
        // the result at index 0,
        // the (encoded) operation at index 1 and
        // the positions from index 2 till the end
        let mut positions: Vec<usize> = group_as_string
            .chars()
            //map operations to ids and insert separators
            .map(|c| match c {
                'c' => ".0.".to_string(),
                '+' => ".1.".to_string(),
                '-' => ".2.".to_string(),
                '*' => ".3.".to_string(),
                ':' => ".4.".to_string(),
                _ => c.to_string(),
            })
            .collect::<String>()
            //Split Res from operation from Positions
            .split(".")
            //try to parse into number
            .map(|xs| match xs.parse::<usize>() {
                Ok(x) => x,
                Err(_) => usize::MAX,
            })
            .collect();

        //Check if there are at least 3 entries and
        // that here where no conversion errors, i.e. no usize::MAX is in the vector
        if positions.len() >= 3
            && positions
                .iter()
                .fold(0, |max, &pos| if pos > max { pos } else { max })
                < usize::MAX
        {
            let result = positions.remove(0);
            let operation = ['c', '+', '-', '*', ':'][positions.remove(0)];

            let mut new_group = Group {
                operation,
                result,
                options: Vec::new(),
                is_already_in_black_list: true,
                //check if all positions are in one line or column, if yes
                //the group is one dimensional
                is_one_dimensional: positions
                    .iter()
                    .map(|p| p / 10) //row
                    .fold(true, |s, p| s && positions[0] / 10 == p)
                    || positions
                        .iter()
                        .map(|p| p % 10) //column
                        .fold(true, |s, p| s && positions[0] % 10 == p),
                positions,
            };
            //only one dimensional fields can get blacklisted
            new_group.is_already_in_black_list = !new_group.is_one_dimensional;
            //use multi_cartesian_product to get all possible combinations with repetition
            new_group.options = (0..new_group.positions.len())
                .map(|_| (1..=dimension))
                .multi_cartesian_product()
                .filter(|option| new_group.is_valid_option(option))
                .collect();

            if new_group.options.len() > 0 {
                return Ok(new_group);
            }
        };

        Err(format!(
            "Can't parse line or no valid options for group found: {}",
            group_as_string
        ))
    }

    /// Create new group from existing group, but with new options
    pub fn copy_with_new_options(
        &self,
        new_options: &Vec<Vec<usize>>,
        new_is_black_listed: bool,
    ) -> Self {
        Group {
            operation: self.operation,
            result: self.result,
            positions: self.positions.clone(),
            is_one_dimensional: self.is_one_dimensional,
            is_already_in_black_list: new_is_black_listed,
            options: new_options.clone(),
        }
    }

    /// Adds the option with index option_nr to the given field
    /// no validation is done
    /// the return value indicates success (true) or failure (false),
    /// i.e. the option_nr is greater than the available options
    pub fn apply_option_to_field(&self, field: &mut Vec<usize>, option_index: usize) {
        self.positions
            .iter()
            .zip(self.options[option_index].iter())
            .for_each(|(&position, &digit)| {
                field[position] = digit;
            });
    }

    /// Validates the options of a group against a given field and blacklist
    /// Inputs:
    ///  * field - the current representation of the puzzle solution
    ///  * blacklist - the current blacklist for the field positions
    ///
    /// Returns:
    ///  * the number of available option of this group after the validation
    ///  * the number of positions of this group
    ///  * a new group with the new valid options attached

    pub fn get_updated_group(
        &self,
        field: &Vec<usize>,
        black_list: &mut BlackList,
    ) -> (usize, usize, Self) {
        //current options to be validated
        let mut new_options = self.options.clone();
        let mut is_black_listed = self.is_already_in_black_list;

        //for each position
        for index in 0..self.positions.len() {
            let column = self.positions[index] % 10;
            let row = self.positions[index] - column;

            //get the black listed digits for the current position
            let mut position_black_list: HashSet<usize> =
                black_list.get_position_black_list(&self.positions[index]);

            //get the existing digits in the col and row of the current position
            //add those digits to the position blacklist

            (row..row + 9)
                .chain((column..90).step_by(10))
                .map(|i| field[i]) //change index to digit
                .filter(|&digit| digit > 0) //get existing values
                .for_each(|digit| drop(position_black_list.insert(digit)));

            //filter out all digits from the positional blacklist
            new_options = new_options
                .into_iter()
                .filter(|option| !position_black_list.contains(&option[index]))
                .collect();
        }

        //Update the blacklist if new unique values for one dimensional group are found
        if !self.is_already_in_black_list && new_options.len() > 1 {
            is_black_listed =
                black_list.check_options_and_update_black_list(&self.positions, &new_options);
        }

        (
            new_options.len(),
            self.positions.len(),
            self.copy_with_new_options(&new_options, is_black_listed),
        )
    }

    /// Validates if candidate is a valid option for a KenKen group, i.e.
    /// contains no duplicates in the same row or column and
    /// fulfills the mathematical operation
    fn is_valid_option(&self, candidate: &Vec<usize>) -> bool {
        let dimension = candidate.len();

        //check that no duplicates in line or column
        if (0..dimension - 1)
            //get all tuples with different indizies, i.e. upper right corner of the cartesian product
            .flat_map(move |pi| (pi + 1..dimension).map(move |di| (pi, di)))
            //only check positions with the same digit
            .filter(|(pi, di)| candidate[*pi] == candidate[*di])
            //check that these positions are not on the same row or column
            .any(|(pi, di)| {
                self.positions[pi] / 10 == self.positions[di] / 10
                    || self.positions[pi] % 10 == self.positions[di] % 10
            })
        {
            return false;
        }

        //checks the numeric calculation
        match self.operation {
            '+' => self.result == candidate.iter().fold(0, |s, x| s + x),
            '*' => self.result == candidate.iter().fold(1, |s, x| s * x),
            '-' => {
                dimension == 2
                    && self.result == (candidate[1] as i32 - candidate[0] as i32).abs() as usize
            }
            ':' => {
                dimension == 2
                    && ((candidate[1] == (self.result * candidate[0]))
                        || (candidate[0] == (self.result * candidate[1])))
            }
            'c' => dimension == 1 && (candidate[0] == self.result),
            _ => false,
        }
    }
}

#[cfg(test)]
mod kk_groups_tests {

    use super::*;

    #[test]
    fn check_new_kenken() {
        let group = Group::new_kenken(4, "6*10.11.20").unwrap();
        assert_eq!(group.is_one_dimensional, false);
        assert_eq!(group.is_already_in_black_list, true);
        assert_eq!(group.positions, vec!(10, 11, 20));
        assert_eq!(group.operation, '*');
        assert_eq!(group.result, 6);
        assert_eq!(
            group.options,
            vec!(
                vec!(1, 2, 3),
                vec!(1, 3, 2),
                vec!(2, 1, 3),
                vec!(2, 3, 1),
                vec!(3, 1, 2),
                vec!(3, 2, 1)
            )
        );

        let group = Group::new_kenken(5, "4-20.30").unwrap();
        assert_eq!(group.is_one_dimensional, true);
        assert_eq!(group.is_already_in_black_list, false);
        assert_eq!(group.positions, vec!(20, 30));
        assert_eq!(group.operation, '-');
        assert_eq!(group.result, 4);
        assert_eq!(group.options, vec!(vec!(1, 5), vec!(5, 1)));

        let group = Group::new_kenken(8, "21+41.42.43").unwrap();
        assert_eq!(group.is_one_dimensional, true);
        assert_eq!(group.is_already_in_black_list, false);
        assert_eq!(group.positions, vec!(41, 42, 43));
        assert_eq!(group.operation, '+');
        assert_eq!(group.result, 21);
        assert_eq!(
            group.options,
            vec!(
                vec!(6, 7, 8),
                vec!(6, 8, 7),
                vec!(7, 6, 8),
                vec!(7, 8, 6),
                vec!(8, 6, 7),
                vec!(8, 7, 6)
            )
        );

        assert_eq!(Group::new_kenken(9, "22/.01.02").is_err(), true);
    }

    #[test]
    fn check_new_sudoku() {
        let positions: Vec<usize> = vec![3, 4];
        let constants: HashSet<usize> = vec![1, 2, 3, 4, 5, 6, 7].into_iter().collect();

        let group = Group::new_sudoku(&positions, &constants).unwrap();
        assert_eq!(group.is_one_dimensional, false);
        assert_eq!(group.is_already_in_black_list, true);
        assert_eq!(group.positions, vec!(3, 4));
        assert_eq!(group.operation, 's');
        assert_eq!(group.result, 0);
        assert_eq!(group.options, vec!(vec!(8, 9), vec!(9, 8)));
    }
}
