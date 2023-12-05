use simple_graph_algorithms::{Graph, algorithms::dijkstra};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let vec = setup_vec(input.as_lines());
    run_djikstra(vec.0, vec.1)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let vec = setup_vec(input.as_lines());
    let larger_map = duplicate_map(vec.0);
    let len = larger_map[0].len();
    run_djikstra(larger_map, len)
}

fn duplicate_map(mut map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut larger_map: Vec<Vec<i32>> = map.clone();
    // First duplicate map in y direction 4 times and increase risk values
    for i in 1..=4 {
        for y in &map {
            larger_map.push(
                y.iter()
                    .map(|x| {
                        let mut new_risk = x + i;
                        if new_risk >= 10 {
                            new_risk -= 9;
                        }
                        new_risk
                    })
                    .collect(),
            );
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

pub fn setup_vec(content: &[&str]) -> (Vec<Vec<i32>>, usize) {
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

fn run_djikstra(vec: Vec<Vec<i32>>, max_x_size: usize) -> i32 {
    println!("Constructing graph...");
    let mut graph = Graph::from(&vec);
    let target = format!("[{}|{}]", max_x_size - 1, vec.len() - 1);
    println!("Target: {}", target);
    let spt = dijkstra(&mut graph, &String::from("[0|0]")).unwrap();
    let result = spt.shortest_distance(&target).unwrap_or(-1);
    //let result = djikstra(graph.node_by_id(String::from("[0|0]")).unwrap(), graph.node_by_id(target).unwrap()).unwrap_or(-1);
    println!("Shortest path: {}", result);
    result
}
