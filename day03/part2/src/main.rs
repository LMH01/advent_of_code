use std::error::Error;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let ogr = get_raiting(&vec, 0, SystemRaiting::OxygenGeneratorRating).get(0).unwrap().clone();
    let csr = get_raiting(&vec, 0, SystemRaiting::CO2ScrubberRating).get(0).unwrap().clone();
    println!("Oxygen Generator Raiting: {:?}", &ogr);
    println!("CO2 Scrubber Raiting: {:?}", &csr);
    // convert values from binary to decimal
    let ogr_int = isize::from_str_radix(&ogr, 2)?;
    let csr_int = isize::from_str_radix(&csr, 2)?;
    println!("Result: {}", ogr_int * csr_int);
    Ok(())
}

enum SystemRaiting {
    OxygenGeneratorRating,
    CO2ScrubberRating,
}

enum Operation {
    KeepZeros,
    KeepOnes,
}

/// Calculates the system raiting for the specified system\
/// 
/// # Arguments
///
/// * 'vec' - The vector that contains the elements
/// * 'index' - At what index the number should be searched (Should always be called with 0 as
/// start value)
/// * 'system_type' - Determines what calculation should be used to determine the raiting
fn get_raiting(vec: &Vec<String>, index: usize, system_type: SystemRaiting) -> Vec<String> {
    let mut accepted: Vec<String> = Vec::new();// this vector will contain all lines that are still valid for the system
    let mut zeros = 0;
    let mut ones = 0;
    for line in vec {// count how many zeros and ones are at the nth position 
        match line.chars().nth(index).unwrap_or('-') {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => (),
        }
    }   
    let operation: Operation;
    match zeros.cmp(&ones) {// check what operation should be performed. This depends on the system type
        std::cmp::Ordering::Greater => {
            match system_type {
                SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepZeros,
                SystemRaiting::CO2ScrubberRating => operation = Operation::KeepOnes,
            }
        },
        std::cmp::Ordering::Less => {
            match system_type {
                SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepOnes,
                SystemRaiting::CO2ScrubberRating => operation = Operation::KeepZeros,
            }
        },
        std::cmp::Ordering::Equal => {
            match system_type {
                SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepOnes,
                SystemRaiting::CO2ScrubberRating => operation = Operation::KeepZeros,
            }
        }
    }
    for line in vec {// check all lines of the vector and perform the selected operation
        if line.chars().nth(index).unwrap_or('-').eq(&'0') && matches!(operation, Operation::KeepZeros) {
            accepted.push(String::from(line)); 
        } else if line.chars().nth(index).unwrap_or('-').eq(&'1') && matches!(operation, Operation::KeepOnes) {
            accepted.push(String::from(line));
        }
    }
    // ''accepted' vector has been filled with the remaining values
    // Recursivly call this function again until only one element remains.
    // The remaining element is the system raiting.
    if accepted.len() == 1 {
        return accepted;  
    } else {
        return get_raiting(&accepted, index+1, system_type);
    }
}
