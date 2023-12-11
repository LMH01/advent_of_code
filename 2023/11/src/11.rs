use simple_graph_algorithms::{Graph, algorithms::dijkstra};

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let larger_space = enlarge_space(input.as_lines());
    let mut graph = construct_graph(&larger_space);
    let galaxies = galaxy_coordinates(&larger_space);
    let mut total_distance = 0;
    let total_galaxies = galaxies.len();
    // this could be optimized to take only half the time by only calculating each pair once
    for (idx, galaxy) in galaxies.iter().enumerate() {
        println!("Calculating spt for galaxy {}/{}-[{}|{}]", idx, total_galaxies, galaxy.0, galaxy.1);
        let spt = dijkstra(&mut graph, &format!("[{}|{}]", galaxy.0, galaxy.1)).unwrap();
        for (inner_idx, inner_galaxy) in galaxies.iter().enumerate() {
            if idx == inner_idx {
                // we don't have to calculate the path to our own galaxy
                continue;
            }
            total_distance += spt.shortest_distance(&format!("[{}|{}]", inner_galaxy.0, inner_galaxy.1)).unwrap();
        }
    }
    total_distance/2
}

//fn part_2(input: aoc::Input) -> impl ToString {
//    0
//}

fn enlarge_space(input: &[&str]) -> Vec<String> {
    let mut chart = Vec::new();
    // fill y spaces
    let mut enlarge_x = Vec::new();
    let mut first_line = true;
    for y in input {
        let mut line_clean = true;
        let mut new_line = String::new();
        for (idx, c) in y.chars().enumerate() {
            // fill enlarge x vector
            if first_line {
                if c == '#' {
                    enlarge_x.push(false);
                } else {
                    enlarge_x.push(true);
                }
            }
            if c == '#' {
                enlarge_x[idx] = false;
                line_clean = false;
            }
            new_line.push(c);
        }
        first_line = false;
        if line_clean {
            // add line two times because it was completely empty
            chart.push(new_line.clone());
        }
        chart.push(new_line)
    }
    // fill x spaces
    let mut new_chart = Vec::new();
    for y in chart {
        let mut new_line = String::new();
        for (idx, c) in y.chars().enumerate() {
            new_line.push(c);
            if enlarge_x[idx] {
                new_line.push(c);
            }
        }
        new_chart.push(new_line)
    }
    new_chart
}

fn construct_graph(input: &Vec<String>) -> Graph<String> {
    let mut vec = Vec::new();
    for line in input {
        let mut new_line = Vec::new();
        for _ in line.chars() {
            new_line.push(1);
        }
        vec.push(new_line);
    }
    Graph::from(&vec)
}

fn galaxy_coordinates(input: &Vec<String>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (y_idx, y) in input.iter().enumerate() {
        for (c_idx, c) in y.chars().enumerate() {
            if c == '#' {
                galaxies.push((c_idx, y_idx));
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod test {

    use crate::enlarge_space;

    #[test]
    fn test_enlarge_space() {
        let input = vec![
            "...#.",
            ".....",
            "..#.."];
        let res = vec![
            ".....#..",
            "........",
            "........",
            "....#..."];
        assert_eq!(enlarge_space(input.as_slice()), res);
    }
}