use std::error::Error;

use adventofcode_lmh01_lib::{read_file, transform_vec};

fn main() -> Result<(), Box<dyn Error>> {
    let vec = transform_vec(read_file("input.txt")?);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for line in vec {
        let mut zeros = 0;
        let mut ones = 0;
        for character in line.chars() {
            match character {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => (),
            }
        }
        match zeros.cmp(&ones) {
            std::cmp::Ordering::Greater => {
                gamma.push('0');
                epsilon.push('1');
            }
            std::cmp::Ordering::Less => {
                gamma.push('1');
                epsilon.push('0');
            }
            std::cmp::Ordering::Equal => (),
        }
    }
    let gamma_as_int = isize::from_str_radix(&gamma, 2)?;
    let epsilon_as_int = isize::from_str_radix(&epsilon, 2)?;
    println!("Gamma rate: {}\nEpsilon rate: {}", gamma, epsilon);
    println!(
        "Gamma rate: {}\nEpsilon rate: {}",
        gamma_as_int, epsilon_as_int
    );
    println!("Result: {}", gamma_as_int * epsilon_as_int);
    Ok(())
}
