use std::collections::{HashMap, HashSet};

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut points = 0;
    for line in input {
        let (winning_numbers, owned_numbers) = construct_sets(line.to_string(), false);
        // check matches
        let mut card_points = 0;
        for i in owned_numbers {
            if !winning_numbers.contains(&i) {
                continue;
            }
            if card_points == 0 {
                card_points = 1;
            } else {
                card_points *= 2;
            }
        }
        points += card_points;
    }
    println!("Total points: {points}");
    points
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut total_cards = 0;
    let mut cards = HashMap::new();
    for (idx, line) in input.as_lines().iter().enumerate() {
        let (winning_numbers, owned_numbers) = construct_sets(line.to_string(), false);

        // check matches
        let mut card_matches = 0;
        for i in owned_numbers {
            if !winning_numbers.contains(&i) {
                continue;
            }
            card_matches += 1;
        }

        // add these copies for each copy of the current card
        let current_card_copies = match cards.get(&(idx + 1)) {
            Some(v) => *v,
            None => 1,
        };
        for _ in 1..=current_card_copies {
            // updated owned cards and add new copies
            for i in 1..=card_matches {
                match cards.get_mut(&(idx + 1 + i)) {
                    Some(v) => *v += 1,
                    None => _ = cards.insert(idx + 1 + i, 2),
                }
            }
            total_cards += 1;
        }
    }
    println!("Total cards: {total_cards}");
    total_cards
}

#[allow(clippy::collapsible_else_if)]
fn construct_sets(mut line: String, demo: bool) -> (HashSet<u32>, HashSet<u32>) {
    // remove duplicated whitespaces (otherwise empty strings may be contained in the vector)
    line = line.replace("   ", " ");
    line = line.replace("  ", " ");
    let chunks = line.split(' ').collect::<Vec<&str>>();
    let mut winning_numbers = HashSet::new();
    let mut owned_numbers = HashSet::new();
    // filter out empty strings

    // construct sets
    for (i, _) in chunks.iter().enumerate().skip(2) {
        // we skip the first two chunks because they are something like //Card 1:
        // comment in this code for the example
        if demo {
            if (2..=6).contains(&i) {
                winning_numbers.insert(chunks[i].parse::<u32>().unwrap());
            } else if (8..=15).contains(&i) {
                owned_numbers.insert(chunks[i].parse::<u32>().unwrap());
            }
        } else {
            if (2..=11).contains(&i) {
                winning_numbers.insert(chunks[i].parse::<u32>().unwrap());
            } else if (13..=37).contains(&i) {
                owned_numbers.insert(chunks[i].parse::<u32>().unwrap());
            }
        }
    }
    (winning_numbers, owned_numbers)
}
