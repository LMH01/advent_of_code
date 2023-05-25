use std::num::ParseIntError;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day05.txt")?;
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day05.txt")?;
    Ok(())
}

enum Instruction {
    TURN_ON((u16, u16), (u16, u16)),
    TURN_OFF((u16, u16), (u16, u16)),
    TOGGLE((u16, u16), (u16, u16)),
}

impl Instruction {
    fn from_str(string: &str) -> Result<Instruction, String> {
        let splits: Vec<&str> = string.split(' ').collect();
        // Instruction is a toggle
        if splits.len() == 4 {
            let start = build_coordinates(splits[1].split(',').collect());
            let end = build_coordinates(splits[3].split(',').collect());
            if start.is_err() || end.is_err() { 
                return Err(format!("Unable to parse coordinates!: {}", start.err().unwrap()));
            }
            return Ok(Instruction::TOGGLE(start.unwrap(), end.unwrap()));
        } else if splits.len() == 5 {
            match splits[1] {
                "on" => {

                },
                "off" => {

                },
                _ => return Err(format!("Unable to parse operation!: {}", splits[1])),
            }
        }
        Err(String::from("Unable to parse string, unexpected input length!"))
    }
}

/// Transformes indexes 0 and 1 into u16 and returns them as tuple
fn build_coordinates(vec: Vec<&str>) -> Result<(u16, u16), ParseIntError> {
    let x = vec[0].parse::<u16>()?;
    let y = vec[1].parse::<u16>()?;
    Ok((x, y))
}
