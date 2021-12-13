use adventofcode_lmh01_lib::read_file;
use miette::{IntoDiagnostic, Result};

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day13_test.txt")?;
    // Determine what the maximum grid size is
    let size = size(&content)?;
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
    print_dots(&dots);
    let folded1 = fold_y(&mut dots, size.0, size.1);
    print_dots(&folded1);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day13_test.txt")?;
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
fn size(input: &Vec<String>) -> Result<(usize, usize)> {
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

fn fold_y(dots: &mut Vec<Vec<bool>>, max_x: usize, max_y: usize) -> Vec<Vec<bool>>{
    let mut folded: Vec<Vec<bool>> = initialize_dots(max_x, (max_y/2)-1);
    for (index_y, line) in dots.iter().enumerate() {
        if index_y < max_y / 2 {
            for (index_x, dot) in line.iter().enumerate() {
                if *dot {
                    set_dot_active(index_x, index_y, &mut folded);
                }
            }
        }
    }
    folded
}
