use std::error::Error;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let fish = get_draw_numbers(vec.get(0).unwrap_or(&String::from("")));
    let mut ages: Ages = Ages {
        x0: 0,
        x1: 0,
        x2: 0,
        x3: 0,
        x4: 0,
        x5: 0,
        x6: 0,
        x7: 0,
        x8: 0,
    };
    // initialize the values of the struct
    for f in &fish {
        match f {
            0 => ages.x0 += 1,
            1 => ages.x1 += 1,
            2 => ages.x2 += 1,
            3 => ages.x3 += 1,
            4 => ages.x4 += 1,
            5 => ages.x5 += 1,
            6 => ages.x6 += 1,
            7 => ages.x7 += 1,
            8 => ages.x8 += 1,
            _ => (),
        }
    }
    for _i in 1..=256 {
        simulate_one_step(&mut ages);
    }
    println!("Fish total: {}", total(&ages));
    println!("Fish age distribution:");
    print_ages(&ages);
    Ok(())
}

#[derive(Debug)]
struct Ages {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
}

fn total(ages: &Ages) -> u64 {
    ages.x0 + ages.x1 + ages.x2 + ages.x3 + ages.x4 + ages.x5 + ages.x6 + ages.x7 + ages.x8
}

fn print_ages(ages: &Ages) {
    println!("[0]: {:15}", ages.x0);
    println!("[1]: {:15}", ages.x1);
    println!("[2]: {:15}", ages.x2);
    println!("[3]: {:15}", ages.x3);
    println!("[4]: {:15}", ages.x4);
    println!("[5]: {:15}", ages.x5);
    println!("[6]: {:15}", ages.x6);
    println!("[7]: {:15}", ages.x7);
    println!("[8]: {:15}", ages.x8);
}

fn simulate_one_step(ages: &mut Ages) {
    let new_fish = ages.x0;
    ages.x0 = ages.x1;
    ages.x1 = ages.x2;
    ages.x2 = ages.x3;
    ages.x3 = ages.x4;
    ages.x4 = ages.x5;
    ages.x5 = ages.x6;
    ages.x6 = ages.x7 + new_fish;
    ages.x7 = ages.x8;
    ages.x8 = new_fish;
}

/// transforms the string to a vector that contains the numbers that are drawn
fn get_draw_numbers(line: &str) -> Vec<u8> {
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
    drawn_numbers
}
