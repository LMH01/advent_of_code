# Advent of Code
This repository contains the code that i'm using to complete the tasks given by the [Advent of Code](https://adventofcode.com/) event.

I'm using [aocli](https://github.com/sncxyz/aocli) for my project structure.

Unfortunately it is [not welcomed to upload the puzzle inputs](https://adventofcode.com/2023/about) so you will need to use your own inputs.

## Usage

First you have to install [aocli](https://github.com/sncxyz/aocli). You can do so by using the following command: `cargo install aocli` (make sure that `~/.cargo/bin` is added to your `$PATH`) or by using the dev-shell in the provided nix flake (activate with `nix develop`). 

Once it is installed and available you will have to set your session token, it can be extracted from the web browser you are using to access the advent of code site. It will be used to fetch your specific input data. The session token needs to be saved in a file named `.session` placed in the root directory.

Once this is done, you can use the following commands (for more information see [aocli](https://github.com/sncxyz/aocli/blob/master/README.md#commands)):

### Get input

`aoc get <YEAR> <DAY>`

### Run solution

`aoc run <YEAR> <DAY>`
