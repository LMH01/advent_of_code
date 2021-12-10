use std::collections::HashMap;

use adventofcode_lmh01_lib::read_file;
use miette::{miette, Result};

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/day8.txt")?;
    let mut output_values: Vec<String> = Vec::new();
    for line in content {
        if let Some(string) = line.split(" | ").nth(1) {
            output_values.push(String::from(string));
        }
    }
    let mut unique_number_of_segments = 0;
    for line in output_values {
        line.split(' ').for_each(|part| {
            match part.len() {
                2 => unique_number_of_segments += 1, // Segment 1
                4 => unique_number_of_segments += 1, // Segment 4
                3 => unique_number_of_segments += 1, // Segment 7
                7 => unique_number_of_segments += 1, // Segment 8
                _ => (),
            }
        });
    }
    println!("Unique number of segments: {}", unique_number_of_segments);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day8.txt")?;
    let mut entries: HashMap<String, String> = HashMap::new();
    let mut output_total = 0;
    for line in content {
        let mut segments = line.split(" | ");
        let mut key = String::new();
        let mut value = String::new();
        if let Some(string) = segments.next() {
            key = String::from(string);
        }
        if let Some(string) = segments.next() {
            value = String::from(string);
        }
        entries.insert(key, value);
    }
    if debug {
        println!("Entries:");
        for (k, v) in &entries {
            println!("{} | {}", &k, &v);
        }
    }
    for (k, v) in entries {
        let parts: Vec<&str> = k.split(' ').collect();
        // Symbolizes how many times a segment is used a, b, c, d, e, f
        let mut segment_mappings: HashMap<char, char> = HashMap::new();
        let mut number_of_segments: HashMap<char, u8> = initialize_map();
        for part in &parts {
            for char in part.chars() {
                match char {
                    'a' => {
                        *number_of_segments.get_mut(&'a').unwrap() += 1;
                    }
                    'b' => {
                        *number_of_segments.get_mut(&'b').unwrap() += 1;
                    }
                    'c' => {
                        *number_of_segments.get_mut(&'c').unwrap() += 1;
                    }
                    'd' => {
                        *number_of_segments.get_mut(&'d').unwrap() += 1;
                    }
                    'e' => {
                        *number_of_segments.get_mut(&'e').unwrap() += 1;
                    }
                    'f' => {
                        *number_of_segments.get_mut(&'f').unwrap() += 1;
                    }
                    'g' => {
                        *number_of_segments.get_mut(&'g').unwrap() += 1;
                    }
                    _ => (),
                }
            }
        }
        // resolve the first 3 mappings
        for (k, v) in &number_of_segments {
            match v {
                4 => {
                    segment_mappings.insert(*k, 'e');
                }
                6 => {
                    segment_mappings.insert(*k, 'b');
                }
                9 => {
                    segment_mappings.insert(*k, 'f');
                }
                _ => (),
            }
        }
        // I know that i can make the code more efficient by for example combining booth resolve
        // methods into one

        // resolve the mappings for a and c
        resolve_ac_mappings(&parts, &number_of_segments, &mut segment_mappings);
        // resolve the mappings for d and g
        resolve_dg_mappings(&parts, &number_of_segments, &mut segment_mappings);
        // print the resolved mappings
        if debug {
            println!("Resolved mappings for:\n\"{}\"", &k);
            for (k, v) in &segment_mappings {
                println!("{} | {}", &k, &v);
            }
        }

        // Create and fill new vector that contains the resolved values.
        // Resolved means that the string now contains the correct segments that should light up.
        let values: Vec<&str> = v.split(' ').collect();
        let mut resolved_values: Vec<String> = Vec::new();
        for value in values {
            resolved_values.push(resolve_input(String::from(value), &segment_mappings));
        }
        if debug {
            println!("Segemnts correct:");
            for value in &resolved_values {
                println!("{}", &value);
            }
        }

        //TODO Better documentation and revisit console output

        // Convert the resolved strings into the correct display number
        let mut output_number = String::new();
        for string in &resolved_values {
            output_number.push(convert_segment_to_number(string)?);
        }
        output_total += &output_number.parse().unwrap();
        if debug {
            println!("Output value: {}", &output_number);
            println!();
        }
    }
    println!("Output total: {}", output_total);
    Ok(())
}

/// Converts one segment string into the coresponding number
fn convert_segment_to_number(input: &str) -> Result<char> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_unstable();
    let sorted = String::from_iter(chars);
    match sorted.as_ref() {
        "abcefg" => Ok('0'),
        "cf" => Ok('1'),
        "acdeg" => Ok('2'),
        "acdfg" => Ok('3'),
        "bcdf" => Ok('4'),
        "abdfg" => Ok('5'),
        "abdefg" => Ok('6'),
        "acf" => Ok('7'),
        "abcdefg" => Ok('8'),
        "abcdfg" => Ok('9'),
        _ => Err(miette!("Unable to convert segment to number. Input is invalid. This is likely cause by a faulty input file.")),
    }
}

/// Uses the segment_mappings to return a string that is mapped correctly
fn resolve_input(input: String, segment_mappings: &HashMap<char, char>) -> String {
    let mut output = String::new();
    input.chars().into_iter().for_each(|c| {
        output.push(*segment_mappings.get(&c).unwrap());
    });
    output
}

fn initialize_map() -> HashMap<char, u8> {
    let mut map: HashMap<char, u8> = HashMap::new();
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    for c in chars {
        map.insert(c, 0);
    }
    map
}

/// Tries to resolve the mappings for a and c
/// parts contains all entries on the left side of the |
/// number_of_segments contains the characters a-g mapped the the number indicating how many times they appeared
/// segemnt_mappings contains the correct mappings for each character a-g to the real segments
fn resolve_ac_mappings(
    parts: &[&str],
    number_of_segments: &HashMap<char, u8>,
    segment_mappings: &mut HashMap<char, char>,
) {
    let mut aight_time_chars: Vec<char> = Vec::new();
    for (k, v) in number_of_segments {
        if *v == 8 {
            aight_time_chars.push(*k);
        }
    }
    let mut times_char_a = 0;
    let mut times_char_b = 0;
    for part in parts {
        if part.len() == 5 {
            if part.contains(|char| *aight_time_chars.get(0).unwrap() == char) {
                times_char_a += 1;
            }
            if part.contains(|char| *aight_time_chars.get(1).unwrap() == char) {
                times_char_b += 1;
            }
        }
    }
    if times_char_a == 3 {
        segment_mappings.insert(*aight_time_chars.get(0).unwrap(), 'a');
        segment_mappings.insert(*aight_time_chars.get(1).unwrap(), 'c');
    } else if times_char_b == 3 {
        segment_mappings.insert(*aight_time_chars.get(1).unwrap(), 'a');
        segment_mappings.insert(*aight_time_chars.get(0).unwrap(), 'c');
    }
}

/// Tries to resolve the mappings for d and g
/// parts contains all entries on the left side of the |
/// number_of_segments contains the characters a-g mapped the the number indicating how many times they appeared
/// segemnt_mappings contains the correct mappings for each character a-g to the real segments
fn resolve_dg_mappings(
    parts: &[&str],
    number_of_segments: &HashMap<char, u8>,
    segment_mappings: &mut HashMap<char, char>,
) {
    let mut seven_time_chars: Vec<char> = Vec::new();
    for (k, v) in number_of_segments {
        if *v == 7 {
            seven_time_chars.push(*k);
        }
    }
    let mut times_char_a = 0;
    let mut times_char_b = 0;
    for part in parts {
        if part.len() == 6 {
            if part.contains(|char| *seven_time_chars.get(0).unwrap() == char) {
                times_char_a += 1;
            }
            if part.contains(|char| *seven_time_chars.get(1).unwrap() == char) {
                times_char_b += 1;
            }
        }
    }
    if times_char_a == 3 {
        segment_mappings.insert(*seven_time_chars.get(0).unwrap(), 'g');
        segment_mappings.insert(*seven_time_chars.get(1).unwrap(), 'd');
    } else if times_char_b == 3 {
        segment_mappings.insert(*seven_time_chars.get(1).unwrap(), 'g');
        segment_mappings.insert(*seven_time_chars.get(0).unwrap(), 'd');
    }
}
