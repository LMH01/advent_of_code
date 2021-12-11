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

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day10.txt")?;
    let mut corrupted_lines = Vec::new();
    for line in &content {
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
                    corrupted_lines.push(line);
                    break;
                }
            }
        }
    }
    let incomplete_lines = get_incomplete_lines(&content, &corrupted_lines);
    let mut completion_scores: Vec<i64> = Vec::new();
    for line in incomplete_lines {
        let mut completion_string = String::new();
        let mut vec: Vec<char> = Vec::new();
        for char in line.chars() {
            if char == '(' || char == '[' || char == '{' || char == '<' {
                vec.push(char);
            } else {
                let last_char = vec.pop().unwrap();
                if (last_char == '(' && char == ')')
                    || (last_char == '[' && char == ']')
                    || (last_char == '{' && char == '}')
                    || (last_char == '<' && char == '>')
                {
                    continue;
                }
            }
        }
        for char in vec.iter().rev() {
            match char {
                '(' => completion_string.push(')'),
                '[' => completion_string.push(']'),
                '{' => completion_string.push('}'),
                '<' => completion_string.push('>'),
                _ => (),
            }
        }
        if debug {
            println!(
                "Completion score: {:012}",
                completion_string_score(&completion_string)
            );
        }
        completion_scores.push(completion_string_score(&completion_string));
    }
    completion_scores.sort_unstable();
    // This statement can panic if completion_scores does contain an odd number of elements
    println!(
        "Middle score: {}",
        completion_scores.get(completion_scores.len() / 2).unwrap()
    );
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

fn get_incomplete_lines(content: &[String], corrupted_lines: &[&String]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in content {
        let mut line_corrupted = false;
        for entry in corrupted_lines {
            if line.eq(*entry) {
                line_corrupted = true;
            }
        }
        if !line_corrupted {
            output.push(String::from(line));
        }
    }
    output
}

fn completion_string_score(string: &str) -> i64 {
    let mut score = 0;
    for char in string.chars() {
        score *= 5;
        match char {
            ')' => score += 1,
            ']' => score += 2,
            '}' => score += 3,
            '>' => score += 4,
            _ => (),
        }
    }
    score
}
