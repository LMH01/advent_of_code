use std::collections::HashMap;

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day03.txt")?;
    let es = engine_schematic(&content);
    let pn = part_numbers(&es);
    println!("Sum of part numbers: {pn}");
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    let content = read_file("input/y2023/day03.txt")?;
    let es = engine_schematic(&content);
    let g = gears(&es);
    println!("Sum gear rations: {g}");
    Ok(())
}

fn engine_schematic(content: &Vec<String>) -> Vec<Vec<char>> {
    let mut es = Vec::new();
    for line in content {
        let mut current = Vec::new();
        for c in line.chars() {
            current.push(c);
        }
        es.push(current);
    }
    es
}

fn part_numbers(es: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for (y_idx, y) in es.iter().enumerate() {
        let mut current_number = String::new();
        for (x_idx, x) in y.iter().enumerate() {
            if x.is_numeric() {
                current_number.push(*x);
            } else {
                // check if a number has been constructed previously
                if current_number.is_empty() {
                    continue;
                }
                // check all positions, that the number spans
                for i in 0..current_number.len() {
                    if check_for_symbols(es, (y_idx, x_idx - 1 - i)) {
                        total += current_number.parse::<u32>().unwrap();
                        break;
                    }
                }
                current_number = String::new();
            }
        }
        // check if a number has been constructed previously
        if current_number.is_empty() {
            continue;
        }
        // check all positions, that the number spans
        for i in 0..current_number.len() {
            if check_for_symbols(es, (y_idx, y.len() - 1 - i)) {
                total += current_number.parse::<u32>().unwrap();
                break;
            }
        }
    }
    total
}

/// Checks the surrounding area for symbols, returns true if symbol was found
fn check_for_symbols(es: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    // previous row
    if pos.0 > 0 {
        // previous row exists
        if contains_symbol(es, (pos.0 - 1, pos.1 - 1)) {
            return true;
        }
        if contains_symbol(es, (pos.0 - 1, pos.1)) {
            return true;
        }
        if contains_symbol(es, (pos.0 - 1, pos.1 + 1)) {
            return true;
        }
    }
    // current row
    if contains_symbol(es, (pos.0, pos.1 - 1)) {
        return true;
    }
    if contains_symbol(es, (pos.0, pos.1 + 1)) {
        return true;
    }
    // next row
    if pos.0 <= es.len() {
        // next row exists
        if contains_symbol(es, (pos.0 + 1, pos.1 - 1)) {
            return true;
        }
        if contains_symbol(es, (pos.0 + 1, pos.1)) {
            return true;
        }
        if contains_symbol(es, (pos.0 + 1, pos.1 + 1)) {
            return true;
        }
    }
    false
}

/// Checks if pos is a valid pos and if it is checks if it is a symbol.
/// Returns true if pos is valid and contains a symbol
fn contains_symbol(es: &[Vec<char>], pos: (usize, usize)) -> bool {
    if let Some(y) = es.get(pos.0) {
        if let Some(x) = y.get(pos.1) {
            return is_symbol(x);
        }
    }
    false
}

fn is_symbol(c: &char) -> bool {
    !c.is_numeric() && *c != '.'
}

fn gears(es: &Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    // Hash map value: .0 = number 1, .1 = number 2, .2 = determines if the corresponding gear was only used twice, otherwise entry is marked as false
    let mut gears: HashMap<(usize, usize), (u32, Option<u32>, bool)> = HashMap::new();
    for (y_idx, y) in es.iter().enumerate() {
        let mut current_number = String::new();
        for (x_idx, x) in y.iter().enumerate() {
            if x.is_numeric() {
                current_number.push(*x);
            } else {
                // check if a number has been constructed previously
                if current_number.is_empty() {
                    continue;
                }
                // check all positions, that the number spans
                for i in 0..current_number.len() {
                    for gear_pos in check_for_gears(es, (y_idx, x_idx - 1 - i)) {
                        let nr = current_number.parse::<u32>().unwrap();
                        println!("Gear pos: ({},{})", gear_pos.0, gear_pos.1);
                        if let Some(value) = gears.get_mut(&gear_pos) {
                            if value.0 == nr {
                                // our current number is already set
                                continue;
                            }
                            if let Some(val2) = value.1 {
                                if val2 == nr {
                                    // our current number is already set
                                    continue;
                                }
                                // Gear is adjacent to more than two numbers, it is thus marked as invalid
                                value.2 = false;
                            } else {
                                value.1 = Some(nr);
                            }
                        } else {
                            // create new entry for our newly detected gear
                            gears.insert(gear_pos, (nr, None, true));
                        }
                    }
                }
                current_number = String::new();
            }
        }
        // check if a number has been constructed previously
        if current_number.is_empty() {
            continue;
        }
        // check all positions, that the number spans
        for i in 0..current_number.len() {
            for gear_pos in check_for_gears(es, (y_idx, y.len() - 1 - i)) {
                if let Some(value) = gears.get_mut(&gear_pos) {
                    if value.1.is_some() {
                        // Gear is adjacent to more than two numbers, it is thus marked as invalid
                        value.2 = false;
                    } else {
                        value.1 = Some(current_number.parse::<u32>().unwrap());
                        current_number = String::new();
                    }
                } else {
                    gears.insert(
                        gear_pos,
                        (current_number.parse::<u32>().unwrap(), None, true),
                    );
                    current_number = String::new();
                }
            }
        }
    }
    for gear in gears.values() {
        if gear.2 {
            if let Some(x) = gear.1 {
                total += gear.0 * x;
            }
        }
    }
    total
}

/// Checks the surrounding area for gears, returns list that contains all adjacent gear positions
fn check_for_gears(es: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut gears = Vec::new();
    // previous row
    if pos.0 > 0 {
        // previous row exists
        if is_gear(es, (pos.0 - 1, pos.1 - 1)) {
            gears.push((pos.0 - 1, pos.1 - 1));
        }
        if is_gear(es, (pos.0 - 1, pos.1)) {
            gears.push((pos.0 - 1, pos.1));
        }
        if is_gear(es, (pos.0 - 1, pos.1 + 1)) {
            gears.push((pos.0 - 1, pos.1 + 1));
        }
    }
    // current row
    if is_gear(es, (pos.0, pos.1 - 1)) {
        gears.push((pos.0, pos.1 - 1));
    }
    if is_gear(es, (pos.0, pos.1 + 1)) {
        gears.push((pos.0, pos.1 + 1));
    }
    // next row
    if pos.0 <= es.len() {
        // next row exists
        if is_gear(es, (pos.0 + 1, pos.1 - 1)) {
            gears.push((pos.0 + 1, pos.1 - 1));
        }
        if is_gear(es, (pos.0 + 1, pos.1)) {
            gears.push((pos.0 + 1, pos.1));
        }
        if is_gear(es, (pos.0 + 1, pos.1 + 1)) {
            gears.push((pos.0 + 1, pos.1 + 1));
        }
    }
    gears
}

fn is_gear(es: &[Vec<char>], pos: (usize, usize)) -> bool {
    if let Some(y) = es.get(pos.0) {
        if let Some(x) = y.get(pos.1) {
            return *x == '*';
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::years::y2023::day03::{check_for_symbols, contains_symbol};

    use super::part_numbers;

    fn setup_test_vec() -> Vec<Vec<char>> {
        let mut es = Vec::new();
        es.push(vec!['.', '.', '.', '.', '.']);
        es.push(vec!['.', '.', '#', '.', '.']);
        es.push(vec!['.', '.', '.', '.', '.']);
        es
    }

    #[test]
    fn test_check_for_symobls() {
        let es = setup_test_vec();
        assert!(check_for_symbols(&es, (0, 1)));
        assert!(check_for_symbols(&es, (0, 2)));
        assert!(check_for_symbols(&es, (0, 3)));
        assert!(check_for_symbols(&es, (1, 1)));
        assert!(check_for_symbols(&es, (1, 3)));
        assert!(check_for_symbols(&es, (2, 1)));
        assert!(check_for_symbols(&es, (2, 2)));
        assert!(check_for_symbols(&es, (2, 3)));
        assert!(!check_for_symbols(&es, (2, 4)));
    }

    #[test]
    fn test_part_numbers() {
        let mut es = Vec::new();
        es.push(vec!['.', '.', '.', '.', '.', '.']);
        es.push(vec!['.', '.', '7', '5', '5', '.']);
        es.push(vec!['.', '*', '.', '.', '.', '.']);
        es.push(vec!['.', '.', '.', '.', '.', '.']);
        assert_eq!(part_numbers(&es), 755);
    }

    #[test]
    fn test_contains_symbol() {
        let es = setup_test_vec();
        assert!(contains_symbol(&es, (1, 2)));
        assert!(!contains_symbol(&es, (2, 4)));
    }
}
