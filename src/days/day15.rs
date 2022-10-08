use adventofcode_lmh01_lib::read_file;
use lmh01_pathfinding::{Graph, djikstra};
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day15.txt")?; 
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut max_x_size = 0;
    let mut index = 0;
    for line in content {
        max_x_size = line.len();
        let mut inner_vec: Vec<i32> = Vec::new();
        for c in line.chars() {
            inner_vec.push(i32::try_from(c.to_digit(10).unwrap()).unwrap());
            index += 1;
        }
        vec.push(inner_vec);
    }
    // In theory this works, but it is way to slow (did not find path after 1 hour of running)
    println!("Constructing graph...");
    let graph = Graph::<String>::from_i32_vec(&vec);
    if debug {
        println!("{}", graph);
    }
    let target = format!("[{}|{}]", max_x_size-1, vec.len()-1);
    println!("Target: {}", target);
    let result = djikstra(graph.node_by_id(String::from("[0|0]")).unwrap(), graph.node_by_id(target).unwrap()).unwrap_or(-1);
    println!("Shortest path: {}", result);
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    Ok(())
}
