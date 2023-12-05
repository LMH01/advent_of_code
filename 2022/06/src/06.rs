use std::collections::{HashSet, VecDeque};

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let p_start = package_start(input[0], 4);
    println!("Package start after at char {}", p_start);
    p_start
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let p_start = package_start(input[0], 14);
    println!("Package start after at char {}", p_start);
    p_start
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
