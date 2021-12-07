use adventofcode_lmh01_lib::{read_file, transform_vec};
use miette::{IntoDiagnostic, Result};

pub fn part1(_debug: bool) -> Result<()> {
    let vec = transform_vec(read_file("input/day3.txt")?);
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
    let gamma_as_int = isize::from_str_radix(&gamma, 2).into_diagnostic()?;
    let epsilon_as_int = isize::from_str_radix(&epsilon, 2).into_diagnostic()?;
    println!("Gamma rate: {}\nEpsilon rate: {}", gamma, epsilon);
    println!(
        "Gamma rate: {}\nEpsilon rate: {}",
        gamma_as_int, epsilon_as_int
    );
    println!("Result: {}", gamma_as_int * epsilon_as_int);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let vec = read_file("input/day3.txt")?;
    let ogr = get_raiting(&vec, 0, SystemRaiting::OxygenGeneratorRating)
        .get(0)
        .unwrap()
        .clone();
    let csr = get_raiting(&vec, 0, SystemRaiting::CO2ScrubberRating)
        .get(0)
        .unwrap()
        .clone();
    println!("Oxygen Generator Raiting: {:?}", &ogr);
    println!("CO2 Scrubber Raiting: {:?}", &csr);
    // convert values from binary to decimal
    let ogr_int = isize::from_str_radix(&ogr, 2).into_diagnostic()?;
    let csr_int = isize::from_str_radix(&csr, 2).into_diagnostic()?;
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
fn get_raiting(vec: &[String], index: usize, system_type: SystemRaiting) -> Vec<String> {
    let mut accepted: Vec<String> = Vec::new(); // this vector will contain all lines that are still valid for the system
    let mut zeros = 0;
    let mut ones = 0;
    for line in vec {
        // count how many zeros and ones are at the nth position
        match line.chars().nth(index).unwrap_or('-') {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => (),
        }
    }
    let operation: Operation;
    match zeros.cmp(&ones) {
        // check what operation should be performed. This depends on the system type
        std::cmp::Ordering::Greater => match system_type {
            SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepZeros,
            SystemRaiting::CO2ScrubberRating => operation = Operation::KeepOnes,
        },
        std::cmp::Ordering::Less => match system_type {
            SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepOnes,
            SystemRaiting::CO2ScrubberRating => operation = Operation::KeepZeros,
        },
        std::cmp::Ordering::Equal => match system_type {
            SystemRaiting::OxygenGeneratorRating => operation = Operation::KeepOnes,
            SystemRaiting::CO2ScrubberRating => operation = Operation::KeepZeros,
        },
    }
    for line in vec {
        // check all lines of the vector and perform the selected operation
        if (line.chars().nth(index).unwrap_or('-').eq(&'0')
            && matches!(operation, Operation::KeepZeros))
            || (line.chars().nth(index).unwrap_or('-').eq(&'1')
                && matches!(operation, Operation::KeepOnes))
        {
            accepted.push(String::from(line));
        }
    }
    // ''accepted' vector has been filled with the remaining values
    // Recursivly call this function again until only one element remains.
    // The remaining element is the system raiting.
    if accepted.len() == 1 {
        accepted
    } else {
        get_raiting(&accepted, index + 1, system_type)
    }
}
