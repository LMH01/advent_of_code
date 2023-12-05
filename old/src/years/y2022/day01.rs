use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2022/day01.txt")?;
    let mut calories = get_calories(&content);
    calories.sort();
    let max = calories.pop().unwrap();
    println!("Max calories: {}", max);
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2022/day01.txt")?;
    let mut calories = get_calories(&content);
    calories.sort();
    let mut total = calories.pop().unwrap();
    total += calories.pop().unwrap();
    total += calories.pop().unwrap();
    println!("Top 3 total calories: {}", total);
    Ok(())
}

/// Returns vector that contains all sums of calories
fn get_calories(content: &Vec<String>) -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for line in content {
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    calories.push(current_calories);
    calories
}
