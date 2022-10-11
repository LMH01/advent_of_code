use std::collections::HashSet;

use adventofcode_lmh01_lib::read_file;
use lmh01_pathfinding::{neighbor_positions};
use miette::Result;

use super::day15::setup_vec;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day9.txt")?;
    let vec = setup_vec(content);
    let lowpoints = lowpoints(&vec.0);
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
    let content = read_file("input/day9.txt")?;
    let vec = setup_vec(content);
    let lowpoints = lowpoints(&vec.0);
    let mut basins = basins(&vec.0, vec.0[0].len(), lowpoints);
    let mut largest_basins = Vec::new();
    basins.sort();
    for i in 1..=3 {
        largest_basins.push(basins.pop().unwrap());
    }
    println!("Largest basins: {:?}", largest_basins);
    let sum = largest_basins.pop().unwrap() * largest_basins.pop().unwrap() * largest_basins.pop().unwrap();
    println!("Sum: {}", sum);
    Ok(())
}

fn lowpoints(input: &Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
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

/// Returns a vector containing the sizes of all basins that where found
fn basins(input: &Vec<Vec<i32>>, max_x_size: usize, lowpoints: Vec<(usize, usize, i32)>) -> Vec<i32> {
    let mut basins = Vec::new();
    // Get basin size of entry
    for entry in lowpoints {
        let mut neighbors = neighbor_positions((entry.0, entry.1), max_x_size, input.len());
        let mut open_neighbors = Vec::new();
        let mut closed_neighbors: HashSet<(usize, usize)> = HashSet::new();
        let mut basin_size = 0;
        // Check neighbors if they are smaller than 9
        for neighbor in neighbors {
            if input[neighbor.1][neighbor.0] != 9 {
                open_neighbors.push(neighbor);
                basin_size += 1;
            }
        }
        while !open_neighbors.is_empty() {
            let neighbor = open_neighbors.pop().unwrap();
            for neighbor_neighbor in neighbor_positions(neighbor, max_x_size, input.len()) {
                if input[neighbor_neighbor.1][neighbor_neighbor.0] != 9 && !closed_neighbors.contains(&neighbor_neighbor) && !open_neighbors.contains(&neighbor_neighbor) {
                    open_neighbors.push(neighbor_neighbor);
                    basin_size += 1;
                }
            }
            closed_neighbors.insert(neighbor);
        }
        basins.push(basin_size);
    }
    basins
}