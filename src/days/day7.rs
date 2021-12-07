use adventofcode_lmh01_lib::{get_draw_numbers, read_file};
use miette::Result;

use std::{cmp::Ordering, u128::MAX as U128_MAX, u32::MAX};

pub fn part1(debug: bool) -> Result<()> {
    let vec = read_file("input/day7.txt")?;
    let crabs = get_draw_numbers::<u32>(vec.get(0).unwrap_or(&String::from("")))?;
    let mut least_fuel_consumption: (u32, u32) = (MAX, MAX);
    let max_number: u32 = *crabs.iter().max().unwrap();
    for i in 1..=max_number {
        let mut current_fuel_consumption = 0;
        for j in &crabs {
            match i.cmp(j) {
                Ordering::Less => current_fuel_consumption += j - i,
                _ => current_fuel_consumption += i - j,
            }
        }
        if least_fuel_consumption.1 > current_fuel_consumption {
            least_fuel_consumption = (i, current_fuel_consumption);
        }
        if debug {
            println!("{:04} | {:07}", i, current_fuel_consumption);
        }
    }
    println!("Fuel consumption: {}", least_fuel_consumption.1);
    println!("Vertical position: {}", least_fuel_consumption.0);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let vec = read_file("input/day7.txt")?;
    let crabs = get_draw_numbers::<u128>(vec.get(0).unwrap_or(&String::from("")))?;
    let mut least_fuel_consumption: (u128, u128) = (U128_MAX, U128_MAX);
    let max_number: u128 = *crabs.iter().max().unwrap();
    for i in 1..=max_number {
        let mut current_fuel_consumption = 0;
        for j in &crabs {
            match i.cmp(j) {
                Ordering::Less => {
                    let distant_apart = j - i;
                    //println!("Distance apart: {} | {}-{}", distant_apart, j, i);
                    for k in 1..=distant_apart {
                        current_fuel_consumption += k;
                    }
                }
                _ => {
                    let distant_apart = i - j;
                    //println!("Distance apart: {} | {}-{}", distant_apart, i, j);
                    for k in 1..=distant_apart {
                        current_fuel_consumption += k;
                    }
                }
            }
        }
        if least_fuel_consumption.1 > current_fuel_consumption {
            least_fuel_consumption = (i, current_fuel_consumption);
        }
        if debug {
            println!("{:04} | {:010}", i, current_fuel_consumption);
        }
    }
    println!("Fuel consumption: {}", least_fuel_consumption.1);
    println!("Vertical position: {}", least_fuel_consumption.0);
    Ok(())
}
