use std::collections::HashSet;

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut nice_strings = 0;
    let vowels = get_vowels();
    for string in input {
        if is_string_nice_part1(&string, &vowels) {
            nice_strings += 1;
        }
    }
    println!("Nice strings: {nice_strings}");
    nice_strings
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut nice_strings = 0;
    for string in input {
        if is_string_nice_part2(&string) {
            nice_strings += 1;
        }
    }
    println!("Nice strings: {nice_strings}");
    nice_strings
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

fn is_string_nice_part1(string: &str, vowels: &HashSet<char>) -> bool {
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

fn is_string_nice_part2(string: &str) -> bool {
    // index 0 = char in the before last iteration
    // index 1 = char in the last iteration
    let mut last_chars = ('\0', '\0');
    // Stores all pairs that have occured in the string
    let mut pairs: HashSet<(char, char)> = HashSet::new();
    // Stores how many different vocals where found, needs at least a size of 3
    let mut first_condition = false;
    let mut second_condition = false;
    for c in string.chars() {
        // Check first condition
        if last_chars.0 == c {
            first_condition = true;
        }
        // Check second condition
        if pairs.contains(&(last_chars.1, c)) {
            second_condition = true;
        }
        pairs.insert((last_chars.0, last_chars.1));
        last_chars.0 = last_chars.1;
        last_chars.1 = c;
    }
    // Check first condition (sandwitched letter)
    if !first_condition {
        return false;
    }
    // Check second condition (two times same two characters (no overlap))
    if !second_condition {
        return false;
    }
    true
}
