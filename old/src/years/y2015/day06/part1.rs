use std::num::ParseIntError;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day06.txt")?;
    let mut instructions = Vec::new();
    let mut board = Board::new();
    for line in content {
        instructions.push(Instruction::from_str(&line).unwrap());
    }
    for instruction in instructions {
        instruction.run(&mut board);
    }
    if debug {
        board.print();
    }
    println!("Turned on lamps: {}", board.count_on());
    Ok(())
}

enum Instruction {
    TurnOn((u16, u16), (u16, u16)),
    TurnOff((u16, u16), (u16, u16)),
    Toggle((u16, u16), (u16, u16)),
}

impl Instruction {
    fn from_str(string: &str) -> Result<Instruction, String> {
        let splits: Vec<&str> = string.split(' ').collect();
        // Instruction is a toggle
        if splits.len() == 4 {
            let coordinates = build_coordinate_tuple(&splits, 1, 3)?;
            return Ok(Instruction::Toggle(coordinates.0, coordinates.1));
        } else if splits.len() == 5 {
            let coordinates = build_coordinate_tuple(&splits, 2, 4)?;
            match splits[1] {
                "on" => {
                    return Ok(Instruction::TurnOn(coordinates.0, coordinates.1));
                }
                "off" => {
                    return Ok(Instruction::TurnOff(coordinates.0, coordinates.1));
                }
                _ => return Err(format!("Unable to parse operation!: {}", splits[1])),
            }
        }
        Err(format!(
            "Unable to parse string, unexpected input length!: {} - {}",
            string,
            splits.len()
        ))
    }

    /// Executes the instructions by modifying the `board` vector.
    fn run(&self, board: &mut Board) {
        match self {
            Instruction::Toggle(start, end) => {
                board.toggle(start, end);
            }
            Instruction::TurnOff(start, end) => {
                board.turn_off(start, end);
            }
            Instruction::TurnOn(start, end) => {
                board.turn_on(start, end);
            }
        }
    }
}

/// Transformes indexes 0 and 1 into u16 and returns them as tuple
fn build_coordinates(vec: Vec<&str>) -> Result<(u16, u16), ParseIntError> {
    let x = vec[0].parse::<u16>()?;
    let y = vec[1].parse::<u16>()?;
    Ok((x, y))
}

#[allow(clippy::type_complexity)]
/// Builds a coordinate tuple from the coordinates located and the two indexes in the `splits` vector.
fn build_coordinate_tuple(
    splits: &[&str],
    idx_start: usize,
    idx_end: usize,
) -> Result<((u16, u16), (u16, u16)), String> {
    let start = build_coordinates(splits[idx_start].split(',').collect());
    let end = build_coordinates(splits[idx_end].split(',').collect());
    if start.is_err() || end.is_err() {
        return Err(format!(
            "Unable to parse coordinates!: {}",
            start.err().unwrap()
        ));
    }
    Ok((start.unwrap(), end.unwrap()))
}

struct Board {
    cells: Vec<Vec<bool>>,
}

impl Board {
    fn new() -> Self {
        let mut cells = Vec::new();
        for _i in 1..=1000 {
            let mut vec = Vec::new();
            for _j in 1..=1000 {
                vec.push(false);
            }
            cells.push(vec);
        }
        Self { cells }
    }

    /// Turns all lamps between start and end on
    fn turn_on(&mut self, start: &(u16, u16), end: &(u16, u16)) {
        for i in start.1..=end.1 {
            for j in start.0..=end.0 {
                self.cells[i as usize][j as usize] = true;
            }
        }
    }

    /// Turns all lamps between start and end off
    fn turn_off(&mut self, start: &(u16, u16), end: &(u16, u16)) {
        for i in start.1..=end.1 {
            for j in start.0..=end.0 {
                self.cells[i as usize][j as usize] = false;
            }
        }
    }

    /// Toggles all lamps between start and end
    fn toggle(&mut self, start: &(u16, u16), end: &(u16, u16)) {
        for i in start.1..=end.1 {
            for j in start.0..=end.0 {
                self.cells[i as usize][j as usize] = !self.cells[i as usize][j as usize];
            }
        }
    }

    /// Counts how many lamps are turned on
    fn count_on(&self) -> u32 {
        let mut count = 0;
        for i in &self.cells {
            for j in i {
                if *j {
                    count += 1;
                }
            }
        }
        count
    }

    /// Prints the board
    fn print(&self) {
        for i in &self.cells {
            for j in i {
                if *j {
                    print!("X");
                } else {
                    print!("O");
                }
            }
            println!();
        }
    }
}
