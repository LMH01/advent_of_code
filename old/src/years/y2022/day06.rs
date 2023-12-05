use std::collections::{HashSet, VecDeque};

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day06.txt")?;
    let p_start = package_start(content.get(0).unwrap(), 4);
    println!("Package start after at char {}", p_start);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day06.txt")?;
    let p_start = package_start(content.get(0).unwrap(), 14);
    println!("Package start after at char {}", p_start);
    Ok(())
}

fn package_start(input: &str, header_length: usize) -> i32 {
    let mut chars = 0;
    let mut latest_chars = VecDeque::new();
    for c in input.chars() {
        chars += 1;
        latest_chars.push_back(c);
        if latest_chars.len() == header_length {
            if unique(&latest_chars) {
                return chars;
            }
            latest_chars.pop_front();
        }
    }
    chars
}

fn unique(input: &VecDeque<char>) -> bool {
    let mut uniques = HashSet::new();
    for c in input {
        if !uniques.insert(c) {
            return false;
        }
    }
    true
}
