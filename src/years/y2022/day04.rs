use adventofcode_lmh01_lib::{read_file, numbers_from_string};
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day04.txt")?;
    let mut i = 0;
    for line in content {
        let numbers = numbers_from_string(&line);
        let a0 = numbers[0];
        let a1 = numbers[1];
        let a: (u32, u32) = (a0, a1);
        let b0 = numbers[2];
        let b1 = numbers[3];
        let b: (u32, u32) = (b0, b1);
        if fully_contained(a, b) {
            i += 1;
        }
    }
    println!("To reconsider: {i}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day04.txt")?;
    Ok(())
}

fn fully_contained(a: (u32, u32), b: (u32, u32)) -> bool {
    if a.0 >= b.0 && a.1 <= b.1 {
        return true;
    } else if b.0 >= a.0 && b.1 <= a.1 {
        return true;
    }
    false
}
