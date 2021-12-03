use std::error::Error;
use std::i32::MAX;

use adventofcode_lmh01_lib::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file("input.txt")?;
    let mut x;
    let mut y = MAX;
    let mut increases = 0;
    for i in content.iter() {
        x = i.to_string().parse::<i32>().unwrap();
        if x > y {
            increases += 1;
        }
        y = x;
        println!("{}", i);
    }
    println!("Total increases: {}", increases);

    Ok(())
}
