use std::{collections::HashMap, i32::MAX};

use adventofcode_lmh01_lib::read_file;
use miette::{IntoDiagnostic, Result};

fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file("input/y2021/day14.txt")?;
    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    init_insertion_rules(&content, &mut input, &mut insertion_rules);
    if debug {
        println!("insertion rules:");
        for (k, v) in &insertion_rules {
            println!("{} -> {}", k, v);
        }
    }
    let mut current_template = input;
    simulate_steps(10, debug, &mut current_template, insertion_rules)?;
    println!("Result: {}", result(current_template));
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    //let content = read_file("input/y2021/day14_test.txt")?;
    //let mut insertion_rules: HashMap<String, String> = HashMap::new();
    //let mut input = String::new();
    //init_insertion_rules(&content, &mut input, &mut insertion_rules);
    //if debug {
    //    println!("insertion rules:");
    //    for (k, v) in &insertion_rules {
    //        println!("{} -> {}", k, v);
    //    }
    //}
    //let mut current_template = input;
    //simulate_steps(40, debug, &mut current_template, insertion_rules)?;
    //println!("Result: {}", result(current_template));
    println!("No solution available for part 2!");
    Ok(())
}

/// Initialize the insertion rules
fn init_insertion_rules(
    content: &[String],
    input: &mut String,
    insertion_rules: &mut HashMap<String, String>,
) {
    for (index, line) in content.iter().enumerate() {
        if index == 0 {
            *input = line.clone();
        } else if index >= 2 {
            if let Some(x) = line.split(" -> ").next() {
                if let Some(y) = line.split(" -> ").nth(1) {
                    insertion_rules.insert(String::from(x), String::from(y));
                }
            }
        }
    }
}

/// Simulate all steps
fn simulate_steps(
    steps: i32,
    debug: bool,
    current_template: &mut String,
    insertion_rules: HashMap<String, String>,
) -> impl ToString {
    for i in 1..=steps {
        if debug {
            println!("Calculating step {}... (with {} elements)", i, current_template.len());
        }
        let mut current_exchanges = String::new();
        for j in 0..=current_template.len() {
            let p1 = match current_template.as_bytes().len() > j {
                true => current_template.as_bytes()[j],
                false => continue,
            };
            let p2 = match current_template.as_bytes().len() > j+1 {
                true => current_template.as_bytes()[j+1],
                false => {
                    current_exchanges.push(p1 as char);
                    break;
                },
            };
            let mut to_search = String::from(p1 as char);
            to_search.push(p2 as char);
            if insertion_rules.contains_key(&to_search) {
                current_exchanges.push(p1 as char);
                current_exchanges.push(
                    insertion_rules
                        .get(&to_search)
                        .unwrap()
                        .parse()
                        .unwrap(),
                );
            }
        }
        *current_template = (*current_exchanges).to_string();
    }
    Ok(())
}

/// Calculate the result
fn result(template: String) -> i32 {
    let abc: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut least: usize = MAX.try_into().unwrap();
    let mut most = 0;
    for letter in abc {
        let count = template.matches(letter).count();
        if least > count && count != 0 {
            least = count;
        }
        if most < count {
            most = count;
        }
    }
    println!("Most common: {}", &most);
    println!("Least common: {}", &least);
    (most - least).try_into().unwrap()
}
