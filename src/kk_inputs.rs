//! Module kk_input provides all functions to load a puzzle from the file system
//!
//! for testing purposes it also contains some "inline" puzzles
//! To use these inline puzzles use the following "filesnames" starting with "Dim"
//!
//! * Dim5 - a 5x5 KenKen
//! * Dim7 - a 7x7 KenKen from [Newdoku id: 8051833](https://newdoku.com/include/online.php?id=8051833)
//! * Dim8 - a 8x8 KenKen from [Newdoku id: 7320085](https://newdoku.com/include/online.php?id=7320085)
//! * Dim9 - a 9x9 KenKen from [Newdoku id: 4379825](https://newdoku.com/include/online.php?id=4379825)
//! * Dim9a - a very "hard" KenKen, solution takes about 25 min.
//! * DimS1 - a sample "Expert" Sudoku
//! * Dim* - default KenKen 4x4 [Newdoku id: 7888719](https://newdoku.com/include/online.php?id=7888719)

use std::fs;

/// Function definition_from_file loads the file with the given name into
/// a vector of strings. Each line in the file is one entry in the vector.
pub fn definition_from_file(name: &str) -> Vec<String>{
    println!("Loading KenKen from file {}", name);

    let contents = fs::read_to_string(name)
        .expect("Something went wrong reading the file.");

    let mut kk:Vec<String> =
        contents.split('\n').map(|c| c.trim().to_string()).collect();

    println!("Loaded {}", kk.remove(0));
    kk

}

/// Function definition_inline copies the inline puzzle with the given name into
/// a vector of strings. Each line in the inline puzzle is one entry in the vector.
pub fn definition_inline(name: &str) -> Vec<String>{
    //open file file_name and read Kenken
        println!("Inline: {}", name);
        let my_kk: Vec<&str>;
        match name {
            "Dim5" =>
                my_kk = vec!["KenKen",
                    "7+00.01.10",
                    "9+02.03.13",
                    "5c04",
                    "40*11.12.21",
                    "2-14.24",
                    "15*20.30",
                    "4*22.32",
                    "2c23",
                    "6+31.40.41",
                    "2:33.34",
                    "12+42.43.44"],
            "Dim7" => //newdoku.com nr. 8051833
                my_kk = vec!["KenKen - 7x7 newdoku.com nr. 8051833",
                    "11+00.01.10",
                    "24*02.03.12",
                    "3*04.05",
                    "42*06.16.26",
                    "17+11.21.22.23",
                    "12+13.14.24",
                    "336*15.25.35.45",
                    "11+20.30.40",
                    "42*31.41.42.43",
                    "15+32.33.34",
                    "10*36.46",
                    "16+44.54.64.63",
                    "28*50.51.60",
                    "5-52.53",
                    "11+55.56.65",
                    "3:61.62",
                    "4c66"
                ],
            "Dim8" => //newdoku.com nr. 7320085
                my_kk = vec!["KenKen - 8x8 newdoku.com nr. 7320085",
                "672*00.01.02.03",
                "2:04.05",
                "23+06.07.15.16",
                "4:10.11",
                "4:12.13",
                "19+14.24.25.35",
                "12*17.27",
                "19+20.21.22.30",
                "6c23",
                "13+26.36.37",
                "4:31.41",
                "1-32.42",
                "13+33.34.43.44",
                "3:40.50",
                "18+45.46.47",
                "20*51.61",
                "8*52.62",
                "12+53.63",
                "14*54.64",
                "10+55.56",
                "2:57.67",
                "1-60.70",
                "11+65.75",
                "8c66",
                "9+71.72",
                "2:73.74",
                "6-76.77"],
            "Dim9" => //newdoku.com nr. 4379825
                my_kk = vec!["KenKen - 9x9 newdoku.com nr. 4379825",
                    "8+00.01",
                    "7+02.03",
                    "2:04.05",
                    "19+06.15.16",
                    "28*07.08.18",
                    "3-10.11",
                    "6+12.22",
                    "9:13.14",
                    "25+17.26.27.36",
                    "3:20.30",
                    "8:21.31",
                    "10+23.24",
                    "8*25.35",
                    "21+28.37.38",
                    "35*32.42",
                    "14+33.34",
                    "288*40.41.50.60",
                    "108*43.53.54",
                    "21+44.45.55.56",
                    "24*46.47.57",
                    "15+48.58.68",
                    "7c51",
                    "17+52.62.63.73",
                    "1134*61.70.71.80",
                    "21+64.74.75",
                    "120*65.66.76",
                    "4c67",
                    "96*72.81.82",
                    "13+77.86.87",
                    "12*78.88",
                    "9+83.84.85"
                ],
            "Dim9a" => //own hard to solve
                my_kk = vec!["KenKen very hard to solve",
                    "20+00.01.10.11",
                    "17+02.03.12",
                    "28+04.05.14.15",
                    "6+06.07.16",
                    "1-08.18",
                    "1c13",
                    "13+17.27",
                    "8+20.21",
                    "16+22.23.32.33",
                    "13+24.25.34",
                    "1-26.36",
                    "23+28.38.48",
                    "23+30.31.40.41",
                    "5+35.45",
                    "3-37.47",
                    "9:42.52",
                    "14+43.44.53",
                    "19+46.56.57.67",
                    "20+50.51.60.61",
                    "4:54.55",
                    "5-58.68",
                    "3-62.63",
                    "18+64.65.66",
                    "3-70.71",
                    "3-72.73",
                    "3-74.75",
                    "7-76.77",
                    "3:78.88",
                    "23+80.81.82.83",
                    "3-84.85",
                    "2-86.87"],
            "DimS1" =>
                my_kk = vec!["Sudoku - Expert",
                    "---.-5-.--6",
                    "---.--1.4--",
                    "-12.---.--9",
                    "8--.---.---",
                    "7--.--6.-5-",
                    "-4-.---.87-",
                    "--3.4--.---",
                    "9-4.6--.---",
                    "---.785.---"
                ],
            _ => //Default 4x4 newdoku.com nr. 7888719
                my_kk = vec!["KenKen - 4x4 newdoku.com nr. 7888719",
                    "8+00.10.11",
                    "5+01.02",
                    "8*03.13.23",
                    "6+12.22.32",
                    "2:20.21",
                    "4*30.31",
                    "3c33"],
        };


    my_kk.iter().map(|c| c.to_string()).collect()

}