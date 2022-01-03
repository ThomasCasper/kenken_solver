use crate::GenArgs;

use rand::prelude::*;
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct GeneratedPuzzle {
    dimension: usize,
    difficulty: usize,
    operations_range: usize,
    solution: Vec<usize>,
    groups: Vec<Vec<usize>>,
    operations: Vec<char>,
    results: Vec<usize>,
}

impl GeneratedPuzzle {
    /// generates a new kenken with a given dimension, difficulty and operations range
    /// Input:
    /// * dimension [3-9] - dimension of the generated KenKen
    /// * difficulty [1-4] - difficulty of the generated Kenken, influences the group sizes
    /// * operations_range [1,2] - only addition (0) or all operations (1) used in the generated KenKen
    pub fn generate_kenken(gen_args: &GenArgs) -> Self {
        //difficulty
        // 0 - easy    up to 9% 1x1fields - max 3-field groups
        // 1 - medium      up to 6% 1x1 fields
        // 2 - hard    up to 3% 1x1 fields - max. 4-field groups
        // 3 - expert  no 1x1 fields

        //operations_range
        // 0 - only +
        // 1 - all operations +-*:

        let mut new_puzzle = GeneratedPuzzle {
            dimension: gen_args.dimension,
            difficulty: gen_args.difficulty,
            operations_range: gen_args.operation_range,
            solution: Vec::new(),
            groups: Vec::new(),
            operations: Vec::new(),
            results: Vec::new(),
        };

        new_puzzle.add_groups();
        new_puzzle.add_solution();
        new_puzzle.add_operations();

        new_puzzle
    }

    /// returns the generated puzzle as a raw string,
    /// which could be saved as an input file for the KenKen solver

    pub fn to_raw_string(&self) -> String {
        let difficulty_names = ["easy", "medium", "hard", "expert"];
        let operation_names = ["with only addition", "with all operations"];

        let mut groups_string = String::new();

        for group_index in 0..self.groups.len() {
            let position_string: String = self.groups[group_index]
                .iter()
                .map(|pos| format!(".{:02}", pos))
                .collect();
            groups_string = format!(
                "{}{}{}{}\n",
                groups_string,
                self.results[group_index],
                self.operations[group_index],
                position_string.chars().skip(1).collect::<String>()
            );
        }

        format!(
            "{} Kenken of dimension {} x {} {}\nKenKen\n{}",
            difficulty_names[self.difficulty],
            self.dimension,
            self.dimension,
            operation_names[self.operations_range],
            groups_string
        )
    }

    fn add_groups(&mut self) {
        let mut rng = thread_rng();
        let dim = self.dimension;
        let mut group_field = [0; 90];
        let mut groups: Vec<Vec<usize>> = vec![Vec::<usize>::new(); dim * dim];

        //fill initial field and groups with 1x1 fields
        (0..dim * dim)
            .map(|group_id| (group_id, 10 * (group_id / dim) + group_id % dim))
            .for_each(|(group_id, position)| {
                groups[group_id].push(position);
                group_field[position] = group_id
            });

        let mut random_index = (0..dim * dim).collect::<Vec<usize>>();
        random_index.shuffle(&mut rng);

        //combine 1x1 groups to larger groups
        //combine dim*dim*(88%+Level*3%) single entry groups
        let mut count_merged_fields: usize = 0;
        let max_merged_fields: usize = dim * dim * (91 + 3 * self.difficulty) / 100;

        for index in random_index {
            if count_merged_fields < max_merged_fields && groups[index].len() == 1 {
                // determine group to merge with current group
                // 0-up, 1-down, 2-left, 3-right
                let mut direction: usize = rng.gen_range(0..4);
                let mut index_to_merge: usize = 0;
                let mut control: usize = 0;
                while control < 4 {
                    if direction < 2 {
                        if direction == 0 && groups[index][0] / 10 == 0 {
                            direction = 1
                        };
                        if direction == 1 && groups[index][0] / 10 == dim - 1 {
                            direction = 0
                        }
                        index_to_merge = group_field[groups[index][0] + direction * 20 - 10];
                    } else {
                        if direction == 2 && groups[index][0] % 10 == 0 {
                            direction = 3
                        };
                        if direction == 3 && groups[index][0] % 10 == dim - 1 {
                            direction = 2
                        }
                        index_to_merge = group_field[groups[index][0] + direction * 2 - 5];
                    }
                    if groups[index_to_merge].len() <= 2 + self.difficulty / 2 {
                        break;
                    };
                    control += 1;
                    direction = (direction + 1) % 4;
                }
                if control < 4 {
                    //merge_groups(index, index_to_merge);
                    if groups[index_to_merge].len() == 1 {
                        count_merged_fields += 2
                    } else {
                        count_merged_fields += 1
                    }

                    let mut append_fields = groups[index_to_merge].clone();
                    groups[index_to_merge]
                        .drain(0..)
                        .for_each(|p| group_field[p] = index);
                    groups[index].append(&mut append_fields);
                }
            }
        }

        for index in 0..groups.len() {
            if !groups[index].is_empty() {
                groups[index].sort();
                self.groups.push(groups[index].clone());
            }
        }
        self.groups.sort();
    }

    /*
        fn check_groups(&self) -> bool {
            let mut group_field: Vec<usize> = vec![0; 90];



                            is_one_dimensional: positions
                        .iter()
                        .map(|p| p / 10) //row
                        .fold(true, |s, p| s && positions[0] / 10 == p)
                        || positions
                            .iter()
                            .map(|p| p % 10) //column
                            .fold(true, |s, p| s && positions[0] % 10 == p),

        }
    */
    fn add_solution(&mut self) {
        let mut rng = thread_rng();
        let dim = self.dimension;

        let mut base_field: Vec<usize> = (0..9)
            .flat_map(|shift| {
                (0..10).map(move |digit| {
                    if shift < dim && digit < dim {
                        (digit + shift) % dim + 1
                    } else {
                        0
                    }
                })
            })
            .collect();

        for _ in 0..100 {
            let direction: usize = 9 * rng.gen_range(1..3) - 8;
            let line1 = rng.gen_range(0..dim);
            let line2 = rng.gen_range(0..dim);
            let mut buf: usize = 1;

            //println!("Dir:  {}, {} <=> {}", direction, line1, line2);
            (0..9)
                .map(|i| direction * i)
                .map(|i| (i + (11 - direction) * line1, i + (11 - direction) * line2))
                .for_each(|(i1, i2)| {
                    buf = base_field[i2];
                    base_field[i2] = base_field[i1];
                    base_field[i1] = buf;
                });
        }

        self.solution = base_field;
    }

    fn add_operations(&mut self) {
        let mut rng = thread_rng();

        for group in &self.groups {
            let digits: Vec<usize> = group
                .iter()
                .map(|&position| self.solution[position])
                .collect();
            let mut operation: char = '+';
            if digits.len() == 1 {
                operation = 'c'
            } else if self.operations_range == 1 {
                let ops_rand = rng.gen_range(0..4);
                if digits.len() == 2 {
                    if digits[0] % digits[1] == 0 || digits[1] % digits[0] == 0 {
                        operation = ':';
                    } else {
                        match ops_rand {
                            0 => operation = '+',
                            1 => operation = '*',
                            _ => operation = '-',
                        };
                    };
                } else {
                    if ops_rand >= 2 {
                        operation = '*'
                    };
                };
            }

            self.operations.push(operation);

            let result: usize;
            match operation {
                'c' => result = digits[0],
                '*' => result = digits.iter().fold(1, |s, d| s * d),
                '-' => {
                    if digits[0] > digits[1] {
                        result = digits[0] - digits[1]
                    } else {
                        result = digits[1] - digits[0]
                    }
                }
                ':' => {
                    if digits[0] > digits[1] {
                        result = digits[0] / digits[1]
                    } else {
                        result = digits[1] / digits[0]
                    }
                }
                _ => result = digits.iter().fold(0, |s, d| s + d),
            };

            self.results.push(result);
        }
    }
}
