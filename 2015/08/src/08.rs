aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut code_chars = 0;
    let mut string_chars = 0;
    for line in input {
        let code_chars_line = line.chars().count();
    }
    code_chars - string_chars
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

fn replace_line(line: String) -> String {
    let mut new_line = String::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {}
            _ => new_line.push(c),
        }
    }
    new_line
}
