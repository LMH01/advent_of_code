use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day03.txt")?;
    let mut priority_total = 0;
    for line in content {
        let split = line.split_at(line.len() / 2);
        let duplicate = duplicate(split.0, split.1);
        let priority = retrieve_priority(duplicate.unwrap());
        priority_total += priority;
    }
    println!("Priority total: {priority_total}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day03.txt")?;
    let mut priority_total = 0;
    let mut current_lines = Vec::new();
    let mut i = 0;
    for line in content {
        i += 1;
        current_lines.push(line);
        if i == 3 {
            let duplicate = common_item(&current_lines[0], &current_lines[1], &current_lines[2]);
            let priority = retrieve_priority(duplicate.unwrap());
            priority_total += priority;
            current_lines = Vec::new();
            i = 0;
        }
    }
    println!("Priority total: {priority_total}");
    Ok(())
}

/// Returns the duplicate
fn duplicate(a: &str, b: &str) -> Option<char> {
    let mut set = HashSet::new();
    for c in a.chars() {
        set.insert(c);
    }
    b.chars().find(|&c| set.contains(&c))
}

/// Finds the only char that is contain in each string
fn common_item(a: &str, b: &str, c: &str) -> Option<char> {
    let mut set = HashSet::new();
    let mut a_b_duplicates = HashSet::new();
    for c in a.chars() {
        set.insert(c);
    }
    for c in b.chars() {
        if set.contains(&c) {
            a_b_duplicates.insert(c);
        }
    }
    c.chars().find(|&c| a_b_duplicates.contains(&c))
}

fn retrieve_priority(c: char) -> i32 {
    // I know that this is not the optimal way but I don't have the time or desire now to
    // figgure out a smarter way.
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => -1,
    }
}
