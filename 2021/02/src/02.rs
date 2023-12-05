aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut horizontal = 0;
    let mut depth = 0;
    for line in input {
        if line.contains("forward") {
            horizontal += replace_line(&line, "forward");
        } else if line.contains("down") {
            depth += replace_line(&line, "down");
        } else if line.contains("up") {
            depth -= replace_line(&line, "up");
        }
    }
    println!("Final horizontal: {}", horizontal);
    println!("Final depth: {}", depth);
    println!("Final result: {}", depth * horizontal);

    depth*horizontal
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input {
        if line.contains("forward") {
            let value = replace_line(&line, "forward");
            horizontal += value;
            depth += aim * value;
        } else if line.contains("down") {
            aim += replace_line(&line, "down");
        } else if line.contains("up") {
            aim -= replace_line(&line, "up");
        }
    }
    println!("Final horizontal: {}", horizontal);
    println!("Final depth: {}", depth);
    println!("Final result: {}", depth * horizontal);

    depth * horizontal
}

fn replace_line(line: &str, to_replace: &str) -> i32 {
    return line
        .replace(to_replace, "")
        .trim()
        .to_string()
        .parse()
        .unwrap_or(0);
}
