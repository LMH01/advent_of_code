use std::i32::MAX;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day1.txt")?;
    let mut x;
    let mut y = MAX;
    let mut increases = 0;
    for i in content.iter() {
        x = i.to_string().parse::<i32>().unwrap();
        if x > y {
            increases += 1;
        }
        y = x;
        if debug {
            println!("{}", i);
        }
    }
    println!("Total increases: {}", increases);

    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day1.txt")?;
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
        if debug {
            print!("{}", current_number);
        }
        match last_number {
            None => {
                if debug {
                    println!(" (N/A - no previous sum)")
                }
            }
            Some(value) => match current_number.cmp(&value) {
                std::cmp::Ordering::Less => {
                    if debug {
                        println!(" (decreased)")
                    }
                }
                std::cmp::Ordering::Greater => {
                    if debug {
                        println!(" (increased)");
                    }
                    increases += 1;
                }
                std::cmp::Ordering::Equal => {
                    if debug {
                        println!(" (no change)")
                    }
                }
            },
        }
        last_number = Some(current_number);
        i += 1;
    }
    println!("Total increases: {}", increases);

    Ok(())
}
