use adventofcode_lmh01_lib::{read_file, read_file_absolute};
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file_absolute("input/y2022/day05.txt")?;
    let tower = parse_input(&content);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day05.txt")?;
    Ok(())
}

/// Parses the puzzles input.
fn parse_input(content: &Vec<String>) -> Vec<Vec<char>> {
    let mut tower: Vec<Vec<char>> = Vec::new();
    let tower_stats = tower_stats(&content);
    println!("Tower stats: {}, {}", tower_stats.0, tower_stats.1);
    for i in 1..=tower_stats.0 {
        tower.push(parse_tower(&content, tower_stats.1, i));
    }
    print_tower(&tower);
    tower
}

/// Returns tuple that contains the number of towers (0) and the maximum height (1).
fn tower_stats(content: &Vec<String>) -> (u32, u32) {
    let mut tower_number = 0;
    let mut tower_line: u32 = 0;
    for line in content {
        if line.is_empty() {
            break;
        }
        tower_line += 1;
    }
    // Line with tower numbers is found
    let line = content.get((tower_line - 1) as usize).unwrap().clone();
    for c in line.chars() {
        if c.is_ascii_digit() {
            let number = c.to_digit(10).unwrap();
            if tower_number < number {
                tower_number = number;
            }
        }
    }
    (tower_number, tower_line - 1)
}

/// Parses a single tower
/// # Arguemnts
/// `max_height` The maximum height of the tower
/// `tower_number` The number of the current tower
fn parse_tower(content: &Vec<String>, max_height: u32, tower_number: u32) -> Vec<char> {
    let mut tower: Vec<char> = Vec::new();
    let char_offset: usize = (1 + (tower_number - 1) * 4).try_into().unwrap();
    for i in (0..max_height).rev() {
        let line = content.get(i as usize).unwrap();
        if let Some(c) = content.get(i as usize).unwrap().chars().nth(char_offset) {
            if !c.is_ascii_alphabetic() {
                break;
            }
            tower.push(c);
        }
    }
    tower
}

fn print_tower(tower: &Vec<Vec<char>>) {
    for (i, v) in tower.iter().enumerate() {
        print!("Tower {}: ", i);
        for c in v {
            print!("{}", c);
        }
        println!();
    }
}
