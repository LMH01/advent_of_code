use std::i32::MAX;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day01.txt")?;
    let line = &content[0];
    let mut floor = 0;
    for c in line.chars() {
        match c {
            '(' => floor +=1,
            ')' => floor -=1,
            _ => (),
        }
    }
    println!("Resulting floor is {floor}");
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day01.txt")?;
    Ok(())
}
