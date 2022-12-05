use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day02.txt")?;
    let mut points = 0;
    for line in content {
        points += match_points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap());
        points += object_points(line.chars().nth(2).unwrap());
    }
    println!("Points: {}", points);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022()(/day02.txt")?;
    Ok(())
}

/// The amount of points I get for the game
fn match_points(opponent: char, me: char) -> i32 {
    match opponent {
        'A' => {
            match me {
                'X' => 3,
                'Y' => 6,
                'Z' => 0,
                _ => 0,
            }
        },
        'B' => {
            match me {
                'X' => 0,
                'Y' => 3,
                'Z' => 6,
                _ => 0,
            }
        }
        'C' => {
            match me {
                'X' => 6,
                'Y' => 0,
                'Z' => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

/// The points for the object
fn object_points(object: char) -> i32 {
    match object {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    }
}
