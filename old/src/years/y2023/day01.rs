use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2023/day01.txt")?;
    let total = get_total(&content);
    println!("Total calibration value: {total}");
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut content = read_file("input/y2023/day01.txt")?;
    //let replaced = replace_string_nr(content);
    replace_string_nr_x(&mut content);
    let total = get_total(&content);
    println!("Total calibration value: {total}");
    Ok(())
}

pub fn replace_string_nr_x(content: &mut Vec<String>) {
    for line in content {
        *line = line.replace("one", "o1e");
        *line = line.replace("two", "t2o");
        *line = line.replace("three", "t3r");
        *line = line.replace("four", "f4r");
        *line = line.replace("five", "f5e");
        *line = line.replace("six", "s6x");
        *line = line.replace("seven", "s7n");
        *line = line.replace("eight", "e8t");
        *line = line.replace("nine", "n9e");
    }
}

fn get_total(content: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in content {
        let mut result = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        }
        total += result.parse().unwrap_or(0);
    }
    total
}
