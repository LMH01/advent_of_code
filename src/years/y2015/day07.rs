use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day07.txt")?;
    let mut instructions = Vec::new();
    for line in content {
        instructions.push(Instruction::from_str(&line));
    }
    // Load values
    //let mut 
    //for instruction in instructions {

    //}
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day07.txt")?;
    Ok(())
}

enum Instruction {
    /// 123 -> x
    Load(u16, String),
    /// x AND y -> z
    And(String, String, String),
    /// x OR y -> Z
    Or(String, String, String),
    /// x LSHIFT 2 -> y
    LShift(String, u8, String),
    /// x RSHIFT 2 -> y
    RShift(String, u8, String),
    /// NOT x -> y
    Not(String, String),
}

impl Instruction {
    fn from_str(string: &str) -> Result<Instruction, String> {
        let splits: Vec<&str> = string.split(' ').collect();
        if splits.len() == 3 {
            // Instruction is load
            let num = splits[0].parse();
            if num.is_err() {
                return Err(format!("Unable to parse input value: {}", num.unwrap_err()));
            }
            return Ok(Instruction::Load(num.unwrap(), String::from(splits[2])));
        } else if splits.len() == 4 {
            // instruction is NOT
            return Ok(Instruction::Not(String::from(splits[1]), String::from(splits[3])));
        } else if splits.len() == 5 {
            match splits[1] {
                "AND" => return Ok(Instruction::And(String::from(splits[0]), String::from(splits[2]), String::from(splits[4]))),
                "OR" => return Ok(Instruction::Or(String::from(splits[0]), String::from(splits[2]), String::from(splits[4]))),
                "LSHIFT" => {
                    let num = splits[2].parse();
                    if num.is_err() {
                        return Err(format!("Unable to parse input value: {}", num.unwrap_err()));
                    }
                    return Ok(Instruction::LShift(String::from(splits[0]), num.unwrap(), String::from(splits[4])))
                },
                "RSHIFT" => {
                    let num = splits[2].parse();
                    if num.is_err() {
                        return Err(format!("Unable to parse input value: {}", num.unwrap_err()));
                    }
                    return Ok(Instruction::RShift(String::from(splits[0]), num.unwrap(), String::from(splits[4])))
                },
                _ => return Err(format!("Unable to parse instruction: Unknown operation: {}", splits[1])),
            }
        }
        Err(format!("Unable to parse string, unexpected input length!: {} - {}", string, splits.len()))
    }
}