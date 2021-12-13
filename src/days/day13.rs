use adventofcode_lmh01_lib::read_file;
use miette::{IntoDiagnostic, Result};

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day13.txt")?;
    // Determine what the maximum grid size is
    let size = get_size(&content)?;
    let mut dots: Vec<Vec<bool>> = initialize_dots(size.0, size.1);
    // Set the active dots
    for line in content {
        if let Some(x) = line.split(",").nth(0) {
            if let Some(y) = line.split(",").nth(1) {
                if debug {
                    println!("Setting dot active: ({}, {})", x, y);
                }
                set_dot_active(
                    x.parse().into_diagnostic()?,
                    y.parse().into_diagnostic()?,
                    &mut dots,
                );
            }
        }
    }
    if debug {
        print_dots(&dots);
    }
    fold_x(&mut dots, size.0, size.1);
    if debug {
        print_dots(&dots);
    }
    println!("Dots visible: {}", visible_dots(&dots));
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day13.txt")?;
    // Determine what the maximum grid size is
    let size = get_size(&content)?;
    let mut dots: Vec<Vec<bool>> = initialize_dots(size.0, size.1);
    let mut folding_directions: Vec<char> = Vec::new();
    // Set the active dots
    for line in content {
        if let Some(x) = line.split(",").nth(0) {
            if let Some(y) = line.split(",").nth(1) {
                if debug {
                    println!("Setting dot active: ({}, {})", x, y);
                }
                set_dot_active(
                    x.parse().into_diagnostic()?,
                    y.parse().into_diagnostic()?,
                    &mut dots,
                );
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
                current_size = (current_size.0/2, current_size.1);
            },
            'y' => {
                fold_y(&mut dots, current_size.0, current_size.1);
                current_size = (current_size.0, current_size.1/2);
            },
            _ => (),
        }
    }
    print_dots(&dots);
    Ok(())
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
fn print_dots(dots: &Vec<Vec<bool>>) {
    println!("Printing:");
    for line in dots {
        for dot in line {
            if *dot {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

/// Sets the input coordinate active
fn set_dot_active(x: usize, y: usize, dots: &mut Vec<Vec<bool>>) {
    *dots.get_mut(y).unwrap().get_mut(x).unwrap() = true;
}

/// Analyses the input file and determies the max x and y coordinate
fn get_size(input: &Vec<String>) -> Result<(usize, usize)> {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in input {
        if let Some(x) = line.split(",").nth(0) {
            if let Some(y) = line.split(",").nth(1) {
                if x.parse::<usize>().into_diagnostic()? > max_x {
                    max_x = x.parse().into_diagnostic()?;
                }
                if y.parse::<usize>().into_diagnostic()? > max_y {
                    max_y = y.parse().into_diagnostic()?;
                }
            }
        }
    }
    Ok((max_x, max_y))
}

/// Fold dots in y direction
fn fold_y(dots: &mut Vec<Vec<bool>>, max_x: usize, max_y: usize) {
    let middle_line = max_y/2;
    let mut folded: Vec<Vec<bool>> = initialize_dots(max_x, middle_line-1);
    for (index_y, line) in dots.iter().enumerate() {
        if index_y < middle_line {
            for (index_x, dot) in line.iter().enumerate() {
                if *dot {
                    set_dot_active(index_x, index_y, &mut folded);
                }
            }
        } else if index_y > middle_line {
            for (index_x, dot) in line.iter().enumerate() {
                if *dot {
                    let distance_to_middle = index_y - middle_line;
                    set_dot_active(index_x, middle_line-distance_to_middle, &mut folded);
                }
            }
        }
    }
    *dots = folded;
}

/// Fold dots in x direction
fn fold_x(dots: &mut Vec<Vec<bool>>, max_x: usize, max_y: usize) {
    let middle_line = max_x/2;
    let mut folded: Vec<Vec<bool>> = initialize_dots(middle_line-1, max_y);
    for (index_y, line) in dots.iter().enumerate() {
        for (index_x, dot) in line.iter().enumerate() {
            if index_x < middle_line {
                if *dot {
                    set_dot_active(index_x, index_y, &mut folded);
                }
            } else if index_x > middle_line {
                if *dot {
                    let distance_to_middle = index_x - middle_line;
                    set_dot_active(middle_line-distance_to_middle, index_y, &mut folded);
                }
            }
        }
    }
    *dots = folded;
}

/// Returns how many dots are set to true in the vector
fn visible_dots(dots: &Vec<Vec<bool>>) -> i32 {
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
