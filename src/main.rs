use crate::days::{day1, day2, day3, day4, day5, day6};
use adventofcode_lmh01_lib::run_day;
use std::{env, error::Error};

pub mod days;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let day: i32 = args[1].parse()?;
        if day > 6 {
            println!("No solution available for day {}", day);
            return Ok(());
        }
        let mut parts: (bool, bool) = (true, true);
        if args.len() >= 3 {
            match args[2].parse::<i32>()? {
                1 => parts = (true, false),
                2 => parts = (false, true),
                _ => {
                    return Err(format!(
                        "Invalid parts argument. Should be 1 or 2, was {}",
                        args[2].parse::<i32>()?
                    )
                    .into())
                }
            }
        }
        match day {
            1 => run_day(day1::part1, day1::part2, 1, parts)?,
            2 => run_day(day2::part1, day2::part2, 2, parts)?,
            3 => run_day(day3::part1, day3::part2, 3, parts)?,
            4 => run_day(day4::part1, day4::part2, 4, parts)?,
            5 => run_day(day5::part1, day5::part2, 5, parts)?,
            6 => run_day(day6::part1, day6::part2, 6, parts)?,
            _ => (),
        }
    } else {
        println!("Running all days...");
        println!();
        run_day(day1::part1, day1::part2, 1, (true, true))?;
        run_day(day2::part1, day2::part2, 2, (true, true))?;
        run_day(day3::part1, day3::part2, 3, (true, true))?;
        run_day(day4::part1, day4::part2, 4, (true, true))?;
        //run_day(day5::part1, day5::part2, 5, (true, true))?;
        run_day(day6::part1, day6::part2, 6, (true, true))?;
    }
    Ok(())
}
