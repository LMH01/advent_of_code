use std::cmp::Ordering;
use std::error::Error;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let mut fish = get_draw_numbers(vec.get(0).unwrap_or(&String::from(""))).unwrap();
    for i in 1..=256 {
        // 80 days
        let mut fish_to_add = 0;
        for f in fish.iter_mut() {
            match f.cmp(&&mut 0) {
                Ordering::Greater => {
                    *f -= 1;
                },
                Ordering::Equal => {
                    *f = 6;
                    fish_to_add += 1;
                },
                Ordering::Less => (),
            }
        }
        for _i in 1..=fish_to_add {
            fish.push(8);
        }
        println!("[Day {:2.0}] Current fish: {:?}", i, fish.len());
    }
    println!("Fish total: {}", fish.len());
    Ok(())
}

/// transforms the string to a vector that contains the numbers that are drawn
fn get_draw_numbers(line: &str) -> Option<Vec<u8>> {
    let mut drawn_numbers = Vec::new();
    let mut current_number: String = String::new();
    for char in line.chars() {
        match char {
            ',' => {
                drawn_numbers.push(current_number.parse::<u8>().unwrap_or(0));
                current_number = String::new();
            }
            _ => current_number.push(char),
        }
    }
    drawn_numbers.push(current_number.parse::<u8>().unwrap_or(0));
    Some(drawn_numbers)
}
