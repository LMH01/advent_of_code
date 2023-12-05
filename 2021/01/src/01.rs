aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut x;
    let mut y = i32::MAX;
    let mut increases = 0;
    for i in input {
        x = i.to_string().parse::<i32>().unwrap();
        if x > y {
            increases += 1;
        }
        y = x;
    }
    println!("Total increases: {}", increases);
    increases
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut increases = 0;
    let mut i = 0;
    let mut last_number = None;
    'outer: while i != input.len() {
        let mut current_number = 0;
        for j in 0..3 {
            let con = input.as_lines().get(i + j);
            if con.is_none() {
                break 'outer;
            } else {
                current_number += input[i + j].to_string().parse::<i32>().unwrap();
            }
        }
        match last_number {
            None => (),
            Some(value) => match current_number.cmp(&value) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => {
                    increases += 1;
                }
                std::cmp::Ordering::Equal => (),
            },
        }
        last_number = Some(current_number);
        i += 1;
    }
    println!("Total increases: {}", increases);
    increases
}
