use std::collections::HashSet;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let vec = setup_vec(input.as_lines());
    let lowpoints = lowpoints(&vec.0);
    let mut risk = 0;
    for point in lowpoints {
        risk += point.2 + 1;
    }
    println!("Total risk level: {}", risk);
    risk
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let vec = setup_vec(input.as_lines());
    let lowpoints = lowpoints(&vec.0);
    let mut basins = basins(&vec.0, vec.0[0].len(), lowpoints);
    let mut largest_basins = Vec::new();
    basins.sort();
    for _i in 1..=3 {
        largest_basins.push(basins.pop().unwrap());
    }
    println!("Largest basins: {:?}", largest_basins);
    let sum = largest_basins.pop().unwrap()
        * largest_basins.pop().unwrap()
        * largest_basins.pop().unwrap();
    println!("Sum: {}", sum);
    sum
}

fn lowpoints(input: &Vec<Vec<i32>>) -> Vec<(usize, usize, i32)> {
    let mut lowpoints: Vec<(usize, usize, i32)> = Vec::new();
    for (i_y, y) in input.iter().enumerate() {
        for (i_x, x) in y.iter().enumerate() {
            let mut lowest_point = true;
            for neighbor in neighbor_positions((i_x, i_y), y.len(), input.len()) {
                if input[neighbor.1][neighbor.0] <= *x {
                    lowest_point = false;
                }
            }
            if lowest_point {
                lowpoints.push((i_x, i_y, *x));
            }
        }
    }
    lowpoints
}

/// Returns a vector containing the sizes of all basins that where found
fn basins(
    input: &Vec<Vec<i32>>,
    max_x_size: usize,
    lowpoints: Vec<(usize, usize, i32)>,
) -> Vec<i32> {
    let mut basins = Vec::new();
    // Get basin size of entry
    for entry in lowpoints {
        let neighbors = neighbor_positions((entry.0, entry.1), max_x_size, input.len());
        let mut open_neighbors = Vec::new();
        let mut closed_neighbors: HashSet<(usize, usize)> = HashSet::new();
        let mut basin_size = 0;
        // Check neighbors if they are smaller than 9
        for neighbor in neighbors {
            if input[neighbor.1][neighbor.0] != 9 {
                open_neighbors.push(neighbor);
                basin_size += 1;
            }
        }
        while let Some(neighbor) = open_neighbors.pop() {
            for neighbor_neighbor in neighbor_positions(neighbor, max_x_size, input.len()) {
                if input[neighbor_neighbor.1][neighbor_neighbor.0] != 9
                    && !closed_neighbors.contains(&neighbor_neighbor)
                    && !open_neighbors.contains(&neighbor_neighbor)
                {
                    open_neighbors.push(neighbor_neighbor);
                    basin_size += 1;
                }
            }
            closed_neighbors.insert(neighbor);
        }
        basins.push(basin_size);
    }
    basins
}

/// Returns the neighboring positions for a position in a 2D graph.
///
/// # Example
/// ```
/// use lmh01_pathfinding::core::neighbor_positions;
///
/// let neighbors = neighbor_positions((2,2), 10, 10);
/// assert_eq!((1, 2), neighbors[0]);
/// assert_eq!((2, 1), neighbors[1]);
/// assert_eq!((3, 2), neighbors[2]);
/// assert_eq!((2, 3), neighbors[3]);
/// ```
pub fn neighbor_positions(
    pos: (usize, usize),
    max_x_size: usize,
    max_y_size: usize,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    if pos.0 != 0 {
        positions.push((pos.0 - 1, pos.1));
    }
    if pos.1 != 0 {
        positions.push((pos.0, pos.1 - 1));
    }
    if pos.0 != max_x_size - 1 {
        positions.push((pos.0 + 1, pos.1));
    }
    if pos.1 != max_y_size - 1 {
        positions.push((pos.0, pos.1 + 1));
    }
    positions
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
