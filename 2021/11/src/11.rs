aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut octopuses: Vec<u8> = Vec::new();
    for line in input {
        for char in line.chars() {
            octopuses.push(char::to_digit(char, 10).unwrap().try_into().unwrap());
        }
    }
    let mut flashes = 0;
    for _i in 1..=100 {
        simulate_step(&mut octopuses, &mut flashes);
    }
    println!("Total flashes: {}", flashes);
    flashes
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut octopuses: Vec<u8> = Vec::new();
    for line in input {
        for char in line.chars() {
            octopuses.push(char::to_digit(char, 10).unwrap().try_into().unwrap());
        }
    }
    for i in 1..=1000 {

        let mut flashes = 0;
        simulate_step(&mut octopuses, &mut flashes);
        if flashes == 100 {
            println!("All octopuses flash in step {}", i);
            return i;
        }
    }
    0
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

fn increase_adjacent_energy_levels(octopus_number: &usize, octopuses: &mut [u8]) {
    // This can probably be solved much cleaner
    let positive_steps;
    let negative_steps;
    // Determine what octopuses are adjacent to the input octopus
    if [19, 29, 39, 49, 59, 69, 79, 89].contains(octopus_number) {
        positive_steps = vec![9, 10];
        negative_steps = vec![1, 10, 11];
    } else if [1, 2, 3, 4, 5, 6, 7, 8].contains(octopus_number) {
        positive_steps = vec![1, 9, 10, 11];
        negative_steps = vec![1];
    } else if [10, 20, 30, 40, 50, 60, 70, 80].contains(octopus_number) {
        positive_steps = vec![1, 10, 11];
        negative_steps = vec![9, 10];
    } else if [91, 92, 93, 94, 95, 96, 97, 98].contains(octopus_number) {
        positive_steps = vec![1];
        negative_steps = vec![1, 9, 10, 11];
    } else if [0].contains(octopus_number) {
        positive_steps = vec![1, 10, 11];
        negative_steps = vec![];
    } else if [9].contains(octopus_number) {
        positive_steps = vec![9, 10];
        negative_steps = vec![1];
    } else if [90].contains(octopus_number) {
        positive_steps = vec![1];
        negative_steps = vec![9, 10];
    } else if [99].contains(octopus_number) {
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
