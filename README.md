# Advent of Code
This repository contains the code that i'm using to complete the tasks given by the [Advent of Code](https://adventofcode.com/) event.

As of 05.12.2022 this repository was changed to contain the solutions for all advent of code years where I participated.

## Usage
If you would like to run all tasks of each year at once just use `cargo run --release -- -a`. Just know that some tasks might take a while to complete. To exclude these use `cargo run --release`.\
Make sure that you use the `--release` flag as some things might take longer without it.

To run a specific day use `cargo run --release -- -y [YEAR] -d [DAY]`\
For example, `cargo run --release -- -y 2021 -d 2` will run all parts of day two of year 2021.

To only run a specific part of a specific day use `cargo run --release -- -y [YEAR] -d [DAY] -p [PART]`.\
For example, `cargo run --release -- -y 2021 -d 2 -p 2` will run only part two of day two of year 2021.
