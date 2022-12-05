use adventofcode_lmh01_lib::read_file;
use lmh01_pathfinding::{Graph, djikstra};
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("../input/y2021/day15.txt")?; 
    let vec = setup_vec(content);
    run_djikstra(vec.0, debug, vec.1);
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("../input/y2021/day15.txt")?; 
    let vec = setup_vec(content);
    let larger_map = duplicate_map(vec.0);
    if debug {
        println!();
        for (i, y) in larger_map.iter().enumerate() {
            for (i_x, x) in y.iter().enumerate() {
                print!("{}", x);
                if i_x % 10 == 9 {
                    print!(" ");
                }
            }
            if i % 10 == 9 {
                println!();
            }
            println!();
        }
    }
    let len = larger_map[0].len();
    run_djikstra(larger_map, debug, len);
    Ok(())
}

fn duplicate_map(mut map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut larger_map: Vec<Vec<i32>> = map.clone();
    // First duplicate map in y direction 4 times and increase risk values
    for i in 1..=4 {
        for y in &map {
            larger_map.push(y.iter().map(|x| {
                let mut new_risk = x + i;
                if new_risk >= 10 {
                    new_risk -= 9;
                }
                new_risk
            }).collect());
        }
    }
    // Now duplicate map in x direction 4 times and increase risk values
    map = larger_map;
    larger_map = Vec::new();
    for y in &map {
        let mut new_line = y.clone();
        for i in 1..=4 {
            for x in y {
                let mut new_risk = x + i;
                if new_risk >= 10 {
                    new_risk -= 9;
                }
                new_line.push(new_risk);
            }
        }
        larger_map.push(new_line);
    }
    larger_map
}

pub fn setup_vec(content: Vec<String>) -> (Vec<Vec<i32>>, usize) {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut max_x_size = 0;
    for line in content {
        max_x_size = line.len();
        let mut inner_vec: Vec<i32> = Vec::new();
        for c in line.chars() {
            inner_vec.push(i32::try_from(c.to_digit(10).unwrap()).unwrap());
        }
        vec.push(inner_vec);
    }
    (vec, max_x_size)
}

fn run_djikstra(vec: Vec<Vec<i32>>, debug: bool, max_x_size: usize) {
    println!("Constructing graph...");
    let graph = Graph::<String>::from_i32_vec(&vec);
    if debug {
        println!("{}", graph);
    }
    let target = format!("[{}|{}]", max_x_size-1, vec.len()-1);
    println!("Target: {}", target);
    let result = djikstra(graph.node_by_id(String::from("[0|0]")).unwrap(), graph.node_by_id(target).unwrap()).unwrap_or(-1);
    println!("Shortest path: {}", result);
}
