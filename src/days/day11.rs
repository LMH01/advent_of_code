use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day11.txt")?;
    let mut octopuses: Vec<u8> = Vec::new();
    for line in content {
        for char in line.chars() {
            octopuses.push(char::to_digit(char, 10).unwrap().try_into().unwrap());
        }
    }
    if debug {
        println!("Start octopusses:");
        print_octopuses(&mut octopuses);
    }
    let mut flashes = 0;
    for i in 1..=100 {
        if debug {
            println!("Simulating step {}...", i);
        }
        simulate_step(&mut octopuses, &mut flashes);
        if debug {
            println!("After {}:", i);
            print_octopuses(&mut octopuses);
        }
    }
    println!("Total flashes: {}", flashes);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day11.txt")?;
    let mut octopuses: Vec<u8> = Vec::new();
    for line in content {
        for char in line.chars() {
            octopuses.push(char::to_digit(char, 10).unwrap().try_into().unwrap());
        }
    }
    if debug {
        println!("Start octopusses:");
        print_octopuses(&mut octopuses);
    }
    for i in 1..=1000 {
        if debug {
            println!("Simulating step {}...", i);
        }

        let mut flashes = 0;
        simulate_step(&mut octopuses, &mut flashes);
        if debug {
            println!("After {}:", i);
            print_octopuses(&mut octopuses);
        }
        if flashes == 100 {
            println!("All octopuses flash in step {}", i);
            break;
        }
    }
    Ok(())
}

fn simulate_step(octopuses: &mut Vec<u8>, flashes: &mut i32) {
    // Increase each energy level by one
    for i in &mut *octopuses {
        *i += 1;
    }

    // Octopus flash
    // Mark what octopuses should flash
    let mut flashed_octopuses: Vec<usize> = Vec::new();
    let mut loop_needed = true;
    while loop_needed {
        loop_needed = false;
        let mut octopuses_to_flash: Vec<usize> = Vec::new();
        // Search for octopus values greater than 9
        for (index, value) in octopuses.iter().enumerate() {
            if *value > 9 && !flashed_octopuses.contains(&index) {
                octopuses_to_flash.push(index);
                loop_needed = true;
            }
        }

        // Set energy values of adjacent octopuses
        for i in octopuses_to_flash {
            increase_adjacent_energy_levels(&i, octopuses);
            flashed_octopuses.push(i);
            *flashes += 1;
        }
    }

    // Set all octopuses with value > 9 to 0
    for i in octopuses {
        if *i > 9 {
            *i = 0;
        }
    }
}

fn increase_adjacent_energy_levels(octopus_number: &usize, octopuses: &mut Vec<u8>) {
    // This can probably be solved much cleaner
    let positive_steps;
    let negative_steps;
    // Determine what octopuses are adjacent to the input octopus
    if vec![19, 29, 39, 49, 59, 69, 79, 89].contains(&octopus_number) {
        positive_steps = vec![9, 10];
        negative_steps = vec![1, 10, 11];
    } else if vec![1, 2, 3, 4, 5, 6, 7, 8].contains(&octopus_number) {
        positive_steps = vec![1, 9, 10, 11];
        negative_steps = vec![1];
    } else if vec![10, 20, 30, 40, 50, 60, 70, 80].contains(&octopus_number) {
        positive_steps = vec![1, 10, 11];
        negative_steps = vec![9, 10];
    } else if vec![91, 92, 93, 94, 95, 96, 97, 98].contains(&octopus_number) {
        positive_steps = vec![1];
        negative_steps = vec![1, 9, 10, 11];
    } else if vec![0].contains(&octopus_number) {
        positive_steps = vec![1, 10, 11];
        negative_steps = vec![];
    } else if vec![9].contains(&octopus_number) {
        positive_steps = vec![9, 10];
        negative_steps = vec![1];
    } else if vec![90].contains(&octopus_number) {
        positive_steps = vec![1];
        negative_steps = vec![9, 10];
    } else if vec![99].contains(&octopus_number) {
        positive_steps = vec![];
        negative_steps = vec![1, 10, 11];
    } else {
        positive_steps = vec![1, 9, 10, 11];
        negative_steps = vec![1, 9, 10, 11];
    }
    let mut octopuses_to_increase: Vec<usize> = Vec::new();
    for i in positive_steps {
        octopuses_to_increase.push(octopus_number + i);
    }
    for i in negative_steps {
        octopuses_to_increase.push(octopus_number - i);
    }
    // Increase the value for all octopuses that are listed in vector octopuses_to_increase
    for (index, value) in octopuses.iter_mut().enumerate() {
        if octopuses_to_increase.contains(&index) {
            *value += 1;
        }
    }
}

fn print_octopuses(octopuses: &mut Vec<u8>) {
    for (index, value) in octopuses.iter().enumerate() {
        match index {
            0 => print!("Line 01: "),
            10 => print!("\nLine 02: "),
            20 => print!("\nLine 03: "),
            30 => print!("\nLine 04: "),
            40 => print!("\nLine 05: "),
            50 => print!("\nLine 06: "),
            60 => print!("\nLine 07: "),
            70 => print!("\nLine 08: "),
            80 => print!("\nLine 09: "),
            90 => print!("\nLine 10: "),
            _ => (),
        }
        print!("{} ", &value);
    }
    println!();
}
