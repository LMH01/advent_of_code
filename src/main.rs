use std::time::Instant;

use crate::years::{y2015, y2021, y2022, y2023};
use adventofcode_lmh01_lib::{run_day, run_slow_day};
use clap::Parser;
use miette::miette;

pub mod years;

#[derive(Parser)]
#[clap(
    about = "Run my solutions for the Advent of Code event",
    long_about = "Run my solutions for the Advent of Code event.\nIf flag --all is not provided only the days that complete in a short period of time are run.\nToo run all days use flag --all."
)]
struct Opts {
    #[clap(
        short,
        long,
        possible_values = ["2015", "2021", "2022", "2023"],
        about  = "Specify what year to run")]
    year: Option<i32>,
    #[clap(short, long, requires = "year", about = "Specify what day to run")]
    day: Option<i32>,
    #[clap(
        short,
        long,
        requires_all = &["day", "year"],
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
        conflicts_with_all = &["day", "part", "year"],
        about = "Run all days of all years, even those that take longer to complete",
    )]
    all: bool,

    #[clap(short, long, about = "Measures and displays the execution time")]
    measure_time: bool,
}

fn main() -> miette::Result<()> {
    let opts = Opts::parse();

    let timer = Instant::now();

    // Handle function calling when day is supplied

    if let Some(year) = opts.year {
        if let Some(day) = opts.day {
            run_year_part(&opts, year, day)?;
        } else {
            run_year(&opts, year)?;
        }
    } else {
        run_year(&opts, 2015)?;
        run_year(&opts, 2021)?;
        run_year(&opts, 2022)?;
        run_year(&opts, 2023)?;
    }
    if opts.measure_time {
        println!("Execution took {:.2?}", timer.elapsed())
    }
    Ok(())
}

/// Runs all days of the specified year
fn run_year(opts: &Opts, year: i32) -> miette::Result<()> {
    println!("Running year {}", year);
    println!();
    match year {
        2015 => {
            run_day(
                y2015::day01::part1,
                y2015::day01::part2,
                1,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day02::part1,
                y2015::day02::part2,
                2,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day03::part1,
                y2015::day03::part2,
                3,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day04::part1,
                y2015::day04::part2,
                4,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day05::part1,
                y2015::day05::part2,
                5,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day06::part1,
                y2015::day06::part2,
                6,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2015::day07::part1,
                y2015::day07::part2,
                7,
                (true, true),
                opts.debug,
            )?;
            Ok(())
        }
        2021 => {
            run_day(
                y2021::day1::part1,
                y2021::day1::part2,
                1,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day2::part1,
                y2021::day2::part2,
                2,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day3::part1,
                y2021::day3::part2,
                3,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day4::part1,
                y2021::day4::part2,
                4,
                (true, true),
                opts.debug,
            )?;
            run_slow_day(
                y2021::day5::part1,
                y2021::day5::part2,
                5,
                (true, true),
                opts.debug,
                opts.all,
            )?;
            run_day(
                y2021::day6::part1,
                y2021::day6::part2,
                6,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day7::part1,
                y2021::day7::part2,
                7,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day8::part1,
                y2021::day8::part2,
                8,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day9::part1,
                y2021::day9::part2,
                9,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day10::part1,
                y2021::day10::part2,
                10,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day11::part1,
                y2021::day11::part2,
                11,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day13::part1,
                y2021::day13::part2,
                13,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day14::part1,
                y2021::day14::part2,
                14,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2021::day15::part1,
                y2021::day15::part2,
                15,
                (true, true),
                opts.debug,
            )?;
            Ok(())
        }
        2022 => {
            run_day(
                y2022::day01::part1,
                y2022::day01::part2,
                1,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2022::day02::part1,
                y2022::day02::part2,
                2,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2022::day03::part1,
                y2022::day03::part2,
                3,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2022::day04::part1,
                y2022::day04::part2,
                4,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2022::day05::part1,
                y2022::day05::part2,
                5,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2022::day06::part1,
                y2022::day06::part2,
                6,
                (true, true),
                opts.debug,
            )?;
            Ok(())
        }
        2023 => {
            run_day(
                y2023::day01::part1,
                y2023::day01::part2,
                1,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2023::day02::part1,
                y2023::day02::part2,
                2,
                (true, true),
                opts.debug,
            )?;
            run_day(
                y2023::day03::part1,
                y2023::day03::part2,
                3,
                (true, true),
                opts.debug,
            )?;
            Ok(())
        }
        _ => Err(miette!(
            "Unable to run: No solution available for year {}.",
            year
        )),
    }
}

fn run_year_part(opts: &Opts, year: i32, day: i32) -> miette::Result<()> {
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
    match year {
        2015 => match day {
            1 => run_day(
                y2015::day01::part1,
                y2015::day01::part2,
                1,
                parts,
                opts.debug,
            )?,
            2 => run_day(
                y2015::day02::part1,
                y2015::day02::part2,
                2,
                parts,
                opts.debug,
            )?,
            3 => run_day(
                y2015::day03::part1,
                y2015::day03::part2,
                3,
                parts,
                opts.debug,
            )?,
            4 => run_day(
                y2015::day04::part1,
                y2015::day04::part2,
                4,
                parts,
                opts.debug,
            )?,
            5 => run_day(
                y2015::day05::part1,
                y2015::day05::part2,
                5,
                parts,
                opts.debug,
            )?,
            6 => run_day(
                y2015::day06::part1,
                y2015::day06::part2,
                6,
                parts,
                opts.debug,
            )?,
            7 => run_day(
                y2015::day07::part1,
                y2015::day07::part2,
                7,
                parts,
                opts.debug,
            )?,
            _ => {
                return Err(miette!(
                    "Unable to run: No solution available for year {} day {}.",
                    &year,
                    &day
                ))
            }
        },
        2021 => match day {
            1 => run_day(y2021::day1::part1, y2021::day1::part2, 1, parts, opts.debug)?,
            2 => run_day(y2021::day2::part1, y2021::day2::part2, 2, parts, opts.debug)?,
            3 => run_day(y2021::day3::part1, y2021::day3::part2, 3, parts, opts.debug)?,
            4 => run_day(y2021::day4::part1, y2021::day4::part2, 4, parts, opts.debug)?,
            5 => run_day(y2021::day5::part1, y2021::day5::part2, 5, parts, opts.debug)?,
            6 => run_day(y2021::day6::part1, y2021::day6::part2, 6, parts, opts.debug)?,
            7 => run_day(y2021::day7::part1, y2021::day7::part2, 7, parts, opts.debug)?,
            8 => run_day(y2021::day8::part1, y2021::day8::part2, 8, parts, opts.debug)?,
            9 => run_day(y2021::day9::part1, y2021::day9::part2, 9, parts, opts.debug)?,
            10 => run_day(
                y2021::day10::part1,
                y2021::day10::part2,
                10,
                parts,
                opts.debug,
            )?,
            11 => run_day(
                y2021::day11::part1,
                y2021::day11::part2,
                11,
                parts,
                opts.debug,
            )?,
            13 => run_day(
                y2021::day13::part1,
                y2021::day13::part2,
                13,
                parts,
                opts.debug,
            )?,
            14 => run_day(
                y2021::day14::part1,
                y2021::day14::part2,
                14,
                parts,
                opts.debug,
            )?,
            15 => run_day(
                y2021::day15::part1,
                y2021::day15::part2,
                15,
                parts,
                opts.debug,
            )?,
            _ => {
                return Err(miette!(
                    "Unable to run: No solution available for year {} day {}.",
                    &year,
                    &day
                ))
            }
        },
        2022 => match day {
            1 => run_day(
                y2022::day01::part1,
                y2022::day01::part2,
                1,
                parts,
                opts.debug,
            )?,
            2 => run_day(
                y2022::day02::part1,
                y2022::day02::part2,
                2,
                parts,
                opts.debug,
            )?,
            3 => run_day(
                y2022::day03::part1,
                y2022::day03::part2,
                3,
                parts,
                opts.debug,
            )?,
            4 => run_day(
                y2022::day04::part1,
                y2022::day04::part2,
                4,
                parts,
                opts.debug,
            )?,
            5 => run_day(
                y2022::day05::part1,
                y2022::day05::part2,
                5,
                parts,
                opts.debug,
            )?,
            6 => run_day(
                y2022::day06::part1,
                y2022::day06::part2,
                6,
                parts,
                opts.debug,
            )?,
            _ => {
                return Err(miette!(
                    "Unable to run: No solution available for year {} day {}.",
                    &year,
                    &day
                ))
            }
        },
        2023 => match day {
            1 => run_day(
                y2023::day01::part1,
                y2023::day01::part2,
                1,
                parts,
                opts.debug,
            )?,
            2 => run_day(
                y2023::day02::part1,
                y2023::day02::part2,
                2,
                parts,
                opts.debug,
            )?,
            3 => run_day(
                y2023::day03::part1,
                y2023::day03::part2,
                3,
                parts,
                opts.debug,
            )?,
            _ => {
                return Err(miette!(
                    "Unable to run: No solution available for year {} day {}.",
                    &year,
                    &day
                ))
            }
        },
        _ => {
            return Err(miette!(
                "Unable to run: Not solution available for year {}.",
                year
            ))
        }
    }
    Ok(())
}
