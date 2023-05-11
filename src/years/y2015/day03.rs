use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day03.txt")?;
    let mut visited_coordinats: HashSet<(i32, i32)> = HashSet::new();
    let line = &content[0];
    let mut x = 0;
    let mut y = 0;
    let mut duplicates = 0;
    for c in line.chars() {
        match c {
            '^' => y+=1,
            '<' => x-=1,
            '>' => x+=1,
            'v' => y-=1,
            _ => (),
        }
        if visited_coordinats.insert((x, y)) {
            duplicates += 1;
        }
    }
    println!("duplicate visits: {duplicates}");
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day03.txt")?;
    Ok(())
}