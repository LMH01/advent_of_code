use std::ops::Range;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2023/day05.txt")?;
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2023/day05.txt")?;
    Ok(())
}

struct Conversion {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
}
