use std::error::Error;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let mut horizontal = 0;
    let mut depth = 0;
    for line in vec {
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

    Ok(())
}

fn replace_line(line: &str, to_replace: &str) -> i32 {
    return line
        .replace(to_replace, "")
        .trim()
        .to_string()
        .parse()
        .unwrap_or(0);
}
