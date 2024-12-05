aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut left_numbers, mut right_numbers) = split_input(input);
    left_numbers.sort();
    right_numbers.sort();
    let mut distances = 0;
    while let Some(number) = left_numbers.pop() {
        distances += number
            .checked_sub(right_numbers.pop().unwrap())
            .unwrap()
            .abs();
    }
    distances
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (mut left_numbers, right_numbers) = split_input(input);
    let mut similarity_score = 0;
    while let Some(number) = left_numbers.pop() {
        let count = right_numbers.iter().filter(|x| *x == &number).count();
        similarity_score += number * count as i32;
    }
    similarity_score
}

fn split_input(input: aoc::Input) -> (Vec<i32>, Vec<i32>) {
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for line in input {
        let mut number = String::new();
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    number.push(c);
                }
                ' ' => {
                    if !number.is_empty() {
                        // first number complete
                        left_numbers.push(number.parse().unwrap());
                        // reset current number
                        number = String::new();
                    }
                }
                _ => (),
            }
        }
        // set last number as right number
        right_numbers.push(number.parse().unwrap());
    }
    (left_numbers, right_numbers)
}
