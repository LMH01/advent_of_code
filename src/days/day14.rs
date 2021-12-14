use std::collections::HashMap;

use adventofcode_lmh01_lib::read_file;
use miette::{IntoDiagnostic, Result};

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day14_test.txt")?;
    let mut insertion_rules: HashMap<String, String> = HashMap::new();
    let mut input = String::new();
    for (index, line) in content.iter().enumerate() {
        if index == 0 {
            input = line.clone();
        } else if index >= 2 {
            if let Some(x) = line.split(" -> ").nth(0) {
                if let Some(y) = line.split(" -> ").nth(1) {
                    insertion_rules.insert(String::from(x), String::from(y));
                }
            }
        }
    }
    if debug {
        println!("insertion rules:");
        for (k, v) in &insertion_rules {
            println!("{} -> {}", k, v);
        }
    }
    let mut current_template = input;
    for i in 1..=1 {
        let mut new_template = current_template.clone();
        let mut new_template2 = String::new();
        let mut last_char: Option<char> = None;
        for (k, v) in &insertion_rules {
            if current_template.contains(k) {
                println!("Current new template: {}", new_template);
                println!("Contains k: {}", k);
                if let Some(p1) = k.chars().nth(0) {
                    if let Some(p2) = k.chars().nth(1) {
                        let insertion = String::from(p1) + v + &String::from(p2);
                        new_template = new_template.replace(k, &insertion);
//                        if let Some(lcc) = insertion.chars().nth(2) {
//                            if let Some(lcp) = last_char {
//                                if lcp == lcc {
//                                    new_template2.push_str(&(String::new() + v + &String::from(p2)));
//                                } else {
//                                    new_template2.push_str(&(String::from(p1) + v + &String::from(p2)));
//                                    last_char = Some(lcc);
//                                }
//                            }
//                        }
//                        new_template.push_str(&insertion);
                    }
                }
                println!("New template: {}", new_template);
            }
        }
        current_template = new_template2;
    }
    println!("Current template: {}", current_template);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/day1.txt")?;
    Ok(())
}
