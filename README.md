# Advent of Code 2021
This repository contains the code that i'm using to complete the tasks given by the [Advent of Code 2021](https://adventofcode.com/2021/) event.

## Usage
If you would like to run all my tasks at once just use `cargo run --release -- -a`. Just know that some tasks might take a while to complete. To exclude these use `cargo run --release`.\
Make sure that you use the `--release` flag as some things might take longer without it.

To run a specific day use `cargo run --release -- -d [DAY]`\
For example, `cargo run --release -- -d 2` will run all parts of day two.

To only run a specific part of a specific day use `cargo run --release -- -d [DAY] -p [PART]`.\
For example, `cargo run --release -- -d 2 -p 2` will run only part two of day two.
