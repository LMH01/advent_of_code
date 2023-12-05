use miette::Result;

mod part1;
mod part2;

pub fn part_1(debug: bool) -> impl ToString {
    part1::part1(debug)
}

pub fn part_2(debug: bool) -> impl ToString {
    part2::part2(debug)
}
