use std::collections::{HashSet, HashMap};

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day04.txt")?;
    let mut points = 0;
    for mut line in content {
        // remove duplicated whitespaces (otherwise empty strings may be contained in the vector)
        line = line.replace("   ", " ");
        line = line.replace("  ", " ");
        let chunks = line.split(' ').collect::<Vec<&str>>();
        let mut winning_numbers = HashSet::new();
        let mut owned_numbers = HashSet::new();
        // filter out empty strings

        // construct sets
        for i in 2..chunks.len() {
            // we skip the first two chunks because they are something like //Card 1:
            if (2..=11).contains(&i) {
                winning_numbers.insert(chunks[i].parse::<u32>().unwrap());
            } else if (13..=37).contains(&i) {
                owned_numbers.insert(chunks[i].parse::<u32>().unwrap());
            }
        }
        // check matches
        let mut card_points = 0;
        for i in owned_numbers {
            if !winning_numbers.contains(&i) {
                continue;
            }
            if card_points == 0 {
                card_points = 1;
            } else {
                card_points = card_points*2;
            }
        }
        println!("Card points: {card_points}");
        points += card_points;
    }
    println!("Total points: {points}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day04.txt")?;
    Ok(())
}

//fn remove_empty(chunks: &mut Vec<&str>) {
//    let mut empty_strings = 0;
//    // determine amount of empty strings
//    for i in 0..chunks.len() {
//        if chunks[i].is_empty() {
//            empty_strings += 1;
//        }
//    }
//    // remove empty strings
//    for _ in 0..=empty_strings {
//        chunks.remove
//    }
//}