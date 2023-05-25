use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day05.txt")?;
    let mut nice_strings = 0;
    let vowels = get_vowels();
    for string in content {
        if is_string_nice(&string, &vowels) {
            nice_strings += 1;
        }
    }
    println!("Nice strings: {nice_strings}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day05.txt")?;
    Ok(())
}

fn get_vowels() -> HashSet<char> {
    let mut set = HashSet::new();
    set.insert('a');
    set.insert('e');
    set.insert('i');
    set.insert('o');
    set.insert('u');
    set
}

fn is_string_nice(string: &str, vowels: &HashSet<char>) -> bool {
    let mut last_char = '\0';
    // Stores how many different vocals where found, needs at least a size of 3
    let mut vowels_contained = 0;
    let mut twice = false;
    for c in string.chars() {
        if vowels.contains(&c) {
            vowels_contained += 1;
        }
        // Check thrid condition
        if last_char == 'a' && c == 'b' {
            return false;
        }
        if last_char == 'c' && c == 'd' {
            return false;
        }
        if last_char == 'p' && c == 'q' {
            return false;
        }
        if last_char == 'x' && c == 'y' {
            return false;
        }
        if last_char == c {
            twice = true;
        }
        last_char = c;
    }
    // Check first condition (at least three vowels)
    if vowels_contained < 3 {
        return false;
    }
    // Check second condition (one latter twice in a row)
    if !twice {
        return false;
    }
    true
}