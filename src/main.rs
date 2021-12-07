use std::time::Instant;

use crate::days::{day1, day2, day3, day4, day5, day6, day7};
use adventofcode_lmh01_lib::{run_day, run_slow_day};
use clap::Parser;
use miette::miette;

pub mod days;

#[derive(Parser)]
#[clap(
    about = "Run my solutions for the Advent of Code event 2021",
    long_about = "Run my solutions for the Advent of Code event 2021.\nIf flag --all is not provided only the days that complete in a short period of time are run.\nToo run all days use flag --all."
)]
struct Opts {
    #[clap(short, long, about = "Specify what day to run")]
    day: Option<i32>,
    #[clap(
        short,
        long,
        requires = "day",
        about = "Specify what part to run",
        long_about = "Specify what part to run. Only works if --day is provided.",
        possible_values = ["1", "2"]
    )]
    part: Option<i32>,
    #[clap(long, about = "Enable debug logging", takes_value = false)]
    debug: bool,

    #[clap(
        short,
        long,
        conflicts_with_all = &["day", "part"],
        about = "Run all days, even those that take longer to complete",
    )]
    all: bool,

    #[clap(
        short,
        long,
        about = "Measures and displays the execution time",
        )]
    measure_time: bool,
}

fn main() -> miette::Result<()> {
    let opts = Opts::parse();

    let timer = Instant::now();

    // Handle function calling when day is supplied

    if let Some(day) = opts.day {
        if !(1..=7).contains(&day) {
            println!("No solution available for day {}", day);
        } else {
            let mut parts: (bool, bool) = (true, true);
            if let Some(part) = opts.part {
                match part {
                    1 => parts = (true, false),
                    2 => parts = (false, true),
                    _ => {
                        return Err(miette!(
                            "Invalid parts argument. Should be 1 or 2. was {}",
                            part
                        ))
                    }
                }
            }
            match day {
                1 => run_day(day1::part1, day1::part2, 1, parts, opts.debug)?,
                2 => run_day(day2::part1, day2::part2, 2, parts, opts.debug)?,
                3 => run_day(day3::part1, day3::part2, 3, parts, opts.debug)?,
                4 => run_day(day4::part1, day4::part2, 4, parts, opts.debug)?,
                5 => run_day(day5::part1, day5::part2, 5, parts, opts.debug)?,
                6 => run_day(day6::part1, day6::part2, 6, parts, opts.debug)?,
                7 => run_day(day7::part1, day7::part2, 7, parts, opts.debug)?,
                _ => (),
            }
        }
    } else {
        println!("Fast only: {}", opts.all);
        println!("Running all days...");
        println!();
        run_day(day1::part1, day1::part2, 1, (true, true), opts.debug)?;
        run_day(day2::part1, day2::part2, 2, (true, true), opts.debug)?;
        run_day(day3::part1, day3::part2, 3, (true, true), opts.debug)?;
        run_day(day4::part1, day4::part2, 4, (true, true), opts.debug)?;
        run_slow_day(
            day5::part1,
            day5::part2,
            5,
            (true, true),
            opts.debug,
            opts.all,
        )?;
        run_day(day6::part1, day6::part2, 6, (true, true), opts.debug)?;
        run_day(day7::part1, day7::part2, 7, (true, true), opts.debug)?;
    }
    if opts.measure_time {
        println!("Execution took {:.2?}", timer.elapsed())
    }
    Ok(())
}
