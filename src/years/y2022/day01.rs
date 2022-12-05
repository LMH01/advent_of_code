use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2022/day01.txt")?;
    let mut max_calories = 0;
    let mut current_calories = 0;
    for line in content {
        if line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }
    if current_calories > max_calories {
        max_calories = current_calories;
    }
    println!("Max calories: {}", max_calories);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let _content = read_file("../input/y2022/day01.txt")?;
    Ok(())
}
