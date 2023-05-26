use miette::Result;

mod part1;
mod part2;

pub fn part1(debug: bool) -> Result<()> {
    part1::part1(debug)
}

pub fn part2(debug: bool) -> Result<()> {
    part2::part2(debug)
}
