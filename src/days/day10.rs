use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/day10.txt")?;
    let mut synatx_error_score = 0;
    for line in content {
        let mut brackets: Vec<char> = Vec::new();
        for char in line.chars() {
            if char == '(' || char == '[' || char == '{' || char == '<' {
                brackets.push(char);
            } else {
                let last_char = brackets.pop().unwrap();
                if (last_char == '(' && char == ')')
                    || (last_char == '[' && char == ']')
                    || (last_char == '{' && char == '}')
                    || (last_char == '<' && char == '>')
                {
                    continue;
                } else {
                    increase_error_score(char, &mut synatx_error_score);
                    break;
                }
            }
        }
    }
    println!("Syntax error score: {}", &synatx_error_score);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    //let content = read_file("input/day10_test.txt")?;
    Ok(())
}

fn increase_error_score(c: char, error_score: &mut i32) {
    match c {
        ')' => *error_score += 3,
        ']' => *error_score += 57,
        '}' => *error_score += 1197,
        '>' => *error_score += 25137,
        _ => (),
    }
}
