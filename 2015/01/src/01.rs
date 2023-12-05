aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let line = &input[0];
    let mut floor = 0;
    for c in line.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn part_2(input: aoc::Input) -> impl ToString {
    let line = &input[0];
    let mut floor = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor < 0 {
                    let char_number = i + 1;
                    return char_number;
                }
            }
            _ => (),
        }
    }
    0
}
