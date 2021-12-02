use std::error::Error;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file("input.txt")?;
    let mut increases = 0;
    let mut i = 0;
    let mut last_number = None;
    'outer: while i != content.len() {
        let mut current_number = 0;
        for j in 0..3 {
            let con = content.get(i + j);
            if con.is_none() {
                break 'outer;
            } else {
                current_number += content
                    .get(i + j)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap();
            }
        }
        print!("{}", current_number);
        match last_number {
            None => println!(" (N/A - no previous sum)"),
            Some(value) => match current_number.cmp(&value) {
                std::cmp::Ordering::Less => println!(" (decreased)"),
                std::cmp::Ordering::Greater => {
                    println!(" (increased)");
                    increases += 1;
                }
                std::cmp::Ordering::Equal => println!(" (no change)"),
            },
        }
        last_number = Some(current_number);
        i += 1;
    }
    println!("Total increases: {}", increases);

    Ok(())
}
