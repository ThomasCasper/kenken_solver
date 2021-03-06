# KenKen_solve solves KenKen and Sudoku puzzles

The puzzle to solve must be specified a separate text-file with the following format

# Kenken:
for more information about KenKen see [KenKen Wikipedia](https://de.wikipedia.org/wiki/Ken_Ken)

## File Format:
* first line comment
* second line: must start with "KenKen" (exactly)
* third line till end of file: the specification of the puzzle
* each line represents one "cell" of the KenKen.
Cell means the connected areas with the given result of an operation
* the format of each line
``` [result][operation][field 1].[field 2]....[field n] ```
* the fields are the coordinates of the fields belonging to the cell,
the left upper corner is 00, the first digit is the row, the second the column
* the operation is one of the following
     * '+' - addition
     * '*' - multiplication
     * '-' - subtraction, the cell must have exactly 2 fields
     * ':' - division, the cell must have exactly 2 fields
     * 'c' - constant, the cell has exactly 1 field with a given digit (which is the result)
 ## Examples
 for the KenKen puzzle [Newdoku puzzle 1278350](https://newdoku.com/include/online.php?id=1278350)
 ```
Newdoku.com KenKen-puzzle nr.: 1278350 with Dim 4 x 4
KenKen
1-00.01
8+02.03.12
6*10.11.20
2-13.23
16*21.30.31
6+22.32.33
```

# Sudoku:
for more information about Sudoku see [Sudoku Wikipedia](https://de.wikipedia.org/wiki/Sudoku)

## File Format:
* first line comment
* second line: must start with "Sudoku" (exactly)
* third line till end of file: the specification of the puzzle
* each line is a row of the Sudoku puzzle,
     * given digits as digits,
     * open fields are represented as "-"
     * for better readability a "." might be entered between 3 position.

##Examples
 for a Sudoku puzzle
 ```
Sudoku Expert level
Sudoku
-5-.--8.269
--2.-43.---
--9.---.---
--7.---.---
---.-9-.-4-
5-3.---.-9-
---.-24.6-5
6--.---.--3
-4-.-8-.---
```