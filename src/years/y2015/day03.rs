use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day03.txt")?;
    let mut visited_coordinates: HashSet<(i32, i32)> = HashSet::new();
    visit_houses(&content[0], &mut visited_coordinates);
    println!("duplicate visits: {}", visited_coordinates.len());
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day03.txt")?;
    let mut visited_coordinates: HashSet<(i32, i32)> = HashSet::new();
    let mut instructions_1 = String::new();
    let mut instructions_2 = String::new();
    for (i, c) in content[0].chars().enumerate() {
        if i%2 == 0 {
            instructions_1.push(c);
        } else {
            instructions_2.push(c);
        }
    }
    visit_houses(&instructions_1, &mut visited_coordinates);
    visit_houses(&instructions_2, &mut visited_coordinates);
    println!("duplicate visits: {}", visited_coordinates.len());
    Ok(())
}

fn visit_houses(instructions: &str, visited_coordinates: &mut HashSet<(i32, i32)>) {
    let mut x = 0;
    let mut y = 0;
    for c in instructions.chars() {
        match c {
            '^' => y+=1,
            '<' => x-=1,
            '>' => x+=1,
            'v' => y-=1,
            _ => (),
        }
        visited_coordinates.insert((x, y));
    }
} 