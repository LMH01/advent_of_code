use adventofcode_lmh01_lib::read_file;

use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let mut draw_numbers: Vec<i32> = Vec::new();
    let mut first_line = true;
    let mut current_bingo_numbers: Vec<i32> = Vec::new();
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    let mut board_number: i32 = 1;
    for line in vec {
        if first_line {
            draw_numbers = get_draw_numbers(&line).unwrap();
            first_line = false;
        } else if line.is_empty() {
            if !current_bingo_numbers.is_empty() {
                bingo_boards.push(BingoBoard::new(current_bingo_numbers, board_number));
                current_bingo_numbers = Vec::new();
                board_number += 1
            }
        } else {
            current_bingo_numbers.append(&mut read_numbers_from_line(&line).unwrap());
        }
    }
    bingo_boards.push(BingoBoard::new(current_bingo_numbers, board_number));
    'outer: for i in draw_numbers {
        for board in bingo_boards.iter_mut() {
            board.add_drawn_number(i);
            match board.is_winner() {
                Some(value) => {
                    if value {
                        println!("Board has won: {:?}", board.board_number);
                        board.print_board();
                        println!("Last number: {}", i);
                        println!("Result: {}", i * board.sum_of_unmarked());
                        break 'outer;
                    }
                }
                None => println!(
                    "Something went wrong while calculating result for board number {}",
                    board.board_number
                ),
            }
        }
    }
    Ok(())
}

fn read_numbers_from_line(line: &str) -> Option<Vec<i32>> {
    let mut drawn_numbers = Vec::new();
    let mut current_number: String = String::new();
    let mut last_char_was_space = false;
    for char in line.chars() {
        match char {
            ' ' => {
                if !last_char_was_space {
                    drawn_numbers.push(current_number.parse::<i32>().unwrap_or(0));
                    current_number = String::new();
                    last_char_was_space = true
                }
            }
            _ => {
                last_char_was_space = false;
                current_number.push(char)
            }
        }
    }
    drawn_numbers.push(current_number.parse::<i32>().unwrap_or(0));
    Some(drawn_numbers)
}

/// transforms the string to a vector that contains the numbers that are drawn
fn get_draw_numbers(line: &str) -> Option<Vec<i32>> {
    let mut drawn_numbers = Vec::new();
    let mut current_number: String = String::new();
    for char in line.chars() {
        match char {
            ',' => {
                drawn_numbers.push(current_number.parse::<i32>().unwrap_or(0));
                current_number = String::new();
            }
            _ => current_number.push(char),
        }
    }
    drawn_numbers.push(current_number.parse::<i32>().unwrap_or(0));
    Some(drawn_numbers)
}

struct BingoBoard {
    board_number: i32,
    /// contains the position of the numbers in the bingo board\
    /// Key = position in bingo board\
    /// Value = Tuple that contains the number at that position and if it has been drawn yet\
    /// Bingo board index numbers:\
    /// ` 0  1  2  3  4`\
    /// ` 5  6  7  8  9`\
    /// `10 11 12 13 14`\
    /// `15 16 17 18 19`\
    /// `20 21 22 23 24`
    numbers: HashMap<i32, (i32, bool)>,
}

impl BingoBoard {
    /// Create a new bingo board
    /// # Arguments
    /// * 'vec' - a vector that contains the numbers of this bingo board. Sorted from top left to bottom right
    fn new(vec: Vec<i32>, board_number: i32) -> Self {
        Self {
            board_number,
            numbers: {
                let mut map: HashMap<i32, (i32, bool)> = HashMap::new();
                for (index, i) in vec.iter().enumerate() {
                    map.insert(index.try_into().unwrap(), (*i, false));
                }
                map
            },
        }
    }

    /// If the bingo board contains the number, it is added to the vector of drawn numbers. Also checks if the board has won after the number has been inserted.
    /// # Returns
    /// Option that contains true if the board has won or false otherwise
    fn add_drawn_number(&mut self, number: i32) -> Option<bool> {
        for (k, v) in self.numbers.clone() {
            if v.0.eq(&number) {
                *self.numbers.get_mut(&k)? = (v.0, true);
            }
        }
        self.is_winner()
    }

    /// Calculates the sum of all bingo board numbers that are unmarked
    fn sum_of_unmarked(&self) -> i32 {
        let mut sum = 0;
        for v in self.numbers.values() {
            if !v.1 {
                sum += v.0;
            }
        }
        sum
    }

    /// Checks if this bingo board is a winner by comparing the values stored in `drawn_numbers` with
    /// the numbers of this bingo board.
    fn is_winner(&self) -> Option<bool> {
        for i in [0, 5, 10, 15, 20] {
            if self.is_line_winner(vec![i, i + 1, i + 2, i + 3, i + 4])? {
                return Some(true);
            }
        }
        for i in [0, 1, 2, 3, 4] {
            if self.is_line_winner(vec![i, i + 5, i + 10, i + 15, i + 20])? {
                return Some(true);
            }
        }
        Some(false)
    }

    fn is_line_winner(&self, indexes: Vec<i32>) -> Option<bool> {
        for i in indexes {
            if !self.numbers.get(&i)?.1 {
                return Some(false);
            }
        }
        Some(true)
    }

    /// Prints the board to the console
    fn print_board(&self) {
        let mut counter = 0;
        println!("Board [{}]", self.board_number);
        for i in 0..25 {
            if counter.eq(&5) {
                println!();
                counter = 0;
            }
            print!("{:02}", &self.numbers.get(&i).unwrap_or(&(-1, false)).0);
            if self.numbers.get(&i).unwrap_or(&(-1, false)).1 {
                print!("[X] ");
            } else {
                print!("[0] ");
            }
            counter += 1;
        }
        println!();
    }
}
