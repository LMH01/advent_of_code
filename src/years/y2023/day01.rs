use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day01.txt")?;
    let mut total = 0;
    for line in content {
        let mut result = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        };
        for c in line.chars().rev() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        }
        total += result.parse().unwrap_or(0);
    }
    println!("Total calibration value: {total}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day01.txt")?;
    Ok(())
}
