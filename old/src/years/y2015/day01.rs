use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2015/day01.txt")?;
    let line = &content[0];
    let mut floor = 0;
    for c in line.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("Resulting floor is {floor}");
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2015/day01.txt")?;
    let line = &content[0];
    let mut floor = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor < 0 {
                    let char_number = i + 1;
                    println!("Santa entered the basement at char {char_number}");
                    break;
                }
            }
            _ => (),
        }
    }
    Ok(())
}
