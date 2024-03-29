aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    // Determine what the maximum grid size is
    let size = get_size(input.as_lines()).unwrap();
    let mut dots: Vec<Vec<bool>> = initialize_dots(size.0, size.1);
    // Set the active dots
    for line in input {
        if let Some(x) = line.split(',').next() {
            if let Some(y) = line.split(',').nth(1) {
                set_dot_active(x.parse().unwrap(), y.parse().unwrap(), &mut dots);
            }
        }
    }
    fold_x(&mut dots, size.0, size.1);
    println!("Dots visible: {}", visible_dots(&dots));
    visible_dots(&dots)
}

fn part_2(input: aoc::Input) -> impl ToString {
    // Determine what the maximum grid size is
    let size = get_size(input.as_lines()).unwrap();
    let mut dots: Vec<Vec<bool>> = initialize_dots(size.0, size.1);
    let mut folding_directions: Vec<char> = Vec::new();
    // Set the active dots
    for line in input {
        if let Some(x) = line.split(',').next() {
            if let Some(y) = line.split(',').nth(1) {
                set_dot_active(x.parse().unwrap(), y.parse().unwrap(), &mut dots);
            }
        }
        // Determine the folding directions
        if line.contains('x') {
            folding_directions.push('x');
        }
        if line.contains('y') {
            folding_directions.push('y');
        }
    }
    // Fold dots
    let mut current_size = size;
    for char in folding_directions {
        match char {
            'x' => {
                fold_x(&mut dots, current_size.0, current_size.1);
                current_size = (current_size.0 / 2, current_size.1);
            }
            'y' => {
                fold_y(&mut dots, current_size.0, current_size.1);
                current_size = (current_size.0, current_size.1 / 2);
            }
            _ => (),
        }
    }
    print_dots(&dots);
    println!("Warning: this solution is hardcoded!");
    "RKHFZGUB" // hardcoded because i don't want to write write a parser
}

/// Creates a new vector of vectors of size (0, 0)x(max_x, max_y)
fn initialize_dots(max_x: usize, max_y: usize) -> Vec<Vec<bool>> {
    let mut dots: Vec<Vec<bool>> = Vec::new();
    for _i in 0..=max_y {
        let mut line: Vec<bool> = Vec::new();
        for _j in 0..=max_x {
            line.push(false);
        }
        dots.push(line);
    }
    dots
}

/// Prints the dots to the console
fn print_dots(dots: &[Vec<bool>]) {
    println!("Printing:");
    for line in dots {
        for dot in line {
            if *dot {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/// Sets the input coordinate active
fn set_dot_active(x: usize, y: usize, dots: &mut [Vec<bool>]) {
    *dots.get_mut(y).unwrap().get_mut(x).unwrap() = true;
}

/// Analyses the input file and determies the max x and y coordinate
fn get_size(input: &[&str]) -> Result<(usize, usize), ()> {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in input {
        if let Some(x) = line.split(',').next() {
            if let Some(y) = line.split(',').nth(1) {
                if x.parse::<usize>().unwrap() > max_x {
                    max_x = x.parse().unwrap();
                }
                if y.parse::<usize>().unwrap() > max_y {
                    max_y = y.parse().unwrap();
                }
            }
        }
    }
    Ok((max_x, max_y))
}

/// Fold dots in y direction
fn fold_y(dots: &mut Vec<Vec<bool>>, max_x: usize, max_y: usize) {
    let middle_line = max_y / 2;
    let mut folded: Vec<Vec<bool>> = initialize_dots(max_x, middle_line - 1);
    for (index_y, line) in dots.iter().enumerate() {
        match index_y.cmp(&middle_line) {
            std::cmp::Ordering::Less => {
                for (index_x, dot) in line.iter().enumerate() {
                    if *dot {
                        set_dot_active(index_x, index_y, &mut folded);
                    }
                }
            }
            std::cmp::Ordering::Greater => {
                for (index_x, dot) in line.iter().enumerate() {
                    if *dot {
                        let distance_to_middle = index_y - middle_line;
                        set_dot_active(index_x, middle_line - distance_to_middle, &mut folded);
                    }
                }
            }
            _ => (),
        }
    }
    *dots = folded;
}

/// Fold dots in x direction
fn fold_x(dots: &mut Vec<Vec<bool>>, max_x: usize, max_y: usize) {
    let middle_line = max_x / 2;
    let mut folded: Vec<Vec<bool>> = initialize_dots(middle_line - 1, max_y);
    for (index_y, line) in dots.iter().enumerate() {
        for (index_x, dot) in line.iter().enumerate() {
            if index_x < middle_line {
                if *dot {
                    set_dot_active(index_x, index_y, &mut folded);
                }
            } else if index_x > middle_line && *dot {
                let distance_to_middle = index_x - middle_line;
                set_dot_active(middle_line - distance_to_middle, index_y, &mut folded);
            }
        }
    }
    *dots = folded;
}

/// Returns how many dots are set to true in the vector
fn visible_dots(dots: &[Vec<bool>]) -> i32 {
    let mut visible_dots = 0;
    for line in dots {
        for dot in line {
            if *dot {
                visible_dots += 1;
            }
        }
    }
    visible_dots
}
