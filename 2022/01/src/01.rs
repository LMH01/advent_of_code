aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut calories = get_calories(input.as_lines());
    calories.sort();
    let max = calories.pop().unwrap();
    println!("Max calories: {}", max);
    max
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut calories = get_calories(input.as_lines());
    calories.sort();
    let mut total = calories.pop().unwrap();
    total += calories.pop().unwrap();
    total += calories.pop().unwrap();
    println!("Top 3 total calories: {}", total);
    total
}

/// Returns vector that contains all sums of calories
fn get_calories(content: &[&str]) -> Vec<i32> {
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
