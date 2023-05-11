use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let input =  String::from("iwrupvqb");
    for i in 1..=i32::MAX {
        let mut to_hash = String::from(&input);
        to_hash.push_str(&i.to_string());
        if hash_and_validate(&to_hash, "00000") {
            println!("Result: {}", i);
            break;
        }
    }
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let input =  String::from("iwrupvqb");
    for i in 1..=i32::MAX {
        let mut to_hash = String::from(&input);
        to_hash.push_str(&i.to_string());
        if hash_and_validate(&to_hash, "000000") {
            println!("Result: {}", i);
            break;
        }
    }
    Ok(())
}

/// Returns true when the input produces a md5 hash that starts with 5 0
fn hash_and_validate(input: &str, delimiter: &str) -> bool {
    let res = md5::compute(input);
    format!("{:?}", res).starts_with(delimiter)
}