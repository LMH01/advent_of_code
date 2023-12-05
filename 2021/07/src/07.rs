use std::{u32::MAX, cmp::Ordering};

use adventofcode_lmh01_lib::get_draw_numbers;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let crabs = get_draw_numbers::<u32>(input[0]).unwrap();
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
    }
    println!("Fuel consumption: {}", least_fuel_consumption.1);
    println!("Vertical position: {}", least_fuel_consumption.0);
    least_fuel_consumption.1
}

fn part_2(input: aoc::Input) -> impl ToString {
    let crabs = get_draw_numbers::<u128>(input[0]).unwrap();
    let mut least_fuel_consumption: (u128, u128) = (u128::MAX, u128::MAX);
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
    }
    println!("Fuel consumption: {}", least_fuel_consumption.1);
    println!("Vertical position: {}", least_fuel_consumption.0);
    least_fuel_consumption.1
}
