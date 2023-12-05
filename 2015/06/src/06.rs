aoc::parts!(1, 2);

mod part1;
mod part2;

pub fn part_1(input: aoc::Input) -> impl ToString {
    part1::part_1(input)
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    part2::part_2(input)
}