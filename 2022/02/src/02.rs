aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut points = 0;
    for line in input {
        points += match_points(line.chars().next().unwrap(), line.chars().nth(2).unwrap());
        points += object_points(line.chars().nth(2).unwrap());
    }
    println!("Points: {}", points);
    points
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut points = 0;
    for line in input {
        let opponent = line.chars().next().unwrap();
        let outcome = line.chars().nth(2).unwrap();
        match opponent {
            'A' => match outcome {
                'X' => points += 3,
                'Y' => points += 4,
                'Z' => points += 8,
                _ => (),
            },
            'B' => match outcome {
                'X' => points += 1,
                'Y' => points += 5,
                'Z' => points += 9,
                _ => (),
            },
            'C' => match outcome {
                'X' => points += 2,
                'Y' => points += 6,
                'Z' => points += 7,
                _ => (),
            },
            _ => (),
        }
    }
    println!("Points: {}", points);
    points
}

/// The amount of points I get for the game
fn match_points(opponent: char, me: char) -> i32 {
    match opponent {
        'A' => match me {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => 0,
        },
        'B' => match me {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => 0,
        },
        'C' => match me {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => 0,
        },
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
