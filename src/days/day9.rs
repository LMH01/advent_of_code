use adventofcode_lmh01_lib::read_file;
use lmh01_pathfinding::{neighbor_positions};
use miette::Result;

use super::day15::setup_vec;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day9.txt")?;
    let vec = setup_vec(content);
    let lowpoints = lowpoints(vec.0);
    let mut risk = 0;
    for point in lowpoints {
        if debug {
            println!("{},{}: {}", point.0, point.1, point.2);
        }
        risk += point.2+1;
    }
    println!("Total risk level: {}", risk);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let _content = read_file("input/day9_test.txt")?; 
    Ok(())
}

fn lowpoints(input: Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
    let mut lowpoints: Vec<(usize, usize, i32)> = Vec::new();
    for (i_y, y) in input.iter().enumerate() {
        for (i_x, x) in y.iter().enumerate() {
            let mut lowest_point = true;
            for neighbor in neighbor_positions((i_x, i_y), y.len(), input.len()) {
                if input[neighbor.1][neighbor.0] <= *x {
                    lowest_point = false;
                }
            }
            if lowest_point {
                lowpoints.push((i_x, i_y, *x));
            }
        }
    }
    lowpoints
}