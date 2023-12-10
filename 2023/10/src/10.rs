use std::thread::current;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut grid = construct_grid(input.as_lines());
    let start = find_start(&grid);
    // I was too lazy to write a parser what the first pipe is, so we have to specify it manually
    // For the two examples it is F, for the real input it is -
    //let c = 'F';
    let c = '-';
    traverse_tunnels(&mut grid, start.unwrap(), c)
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

fn construct_grid(input: &[&str]) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for line in input {
        let mut inner_vec = Vec::new();
        for c in line.chars() {
            inner_vec.push(c);
        }
        vec.push(inner_vec);
    }
    vec
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (y_idx, y) in grid.iter().enumerate() {
        for (x_idx, x) in y.iter().enumerate() {
            if *x == 'S' {
                return Some((x_idx, y_idx))
            }
        }
    }
    None
}

fn traverse_tunnels(grid: &mut Vec<Vec<char>>, start: (usize, usize), start_pipe: char) -> u32 {
    let mut current_r = start;
    let mut prev_r = start;
    let mut current_l = start;
    let mut prev_l = start;
    let mut counter = 0;
    let mut first = true;
    loop {
        if first {
            first = false;
            grid[start.1][start.0] = start_pipe;
            let neighbors = neighboring_pipes(grid, start);
            current_r = neighbors[0];
            current_l = neighbors[1];
            counter += 1;
            continue;
        }
        let next_prev_r = current_r;
        let next_prev_l = current_l;
        current_r = next_pipe(grid, current_r, prev_r);
        current_l = next_pipe(grid, current_l, prev_l);
        prev_r = next_prev_r;
        prev_l = next_prev_l;
        // check if circle is completed
        counter += 1;
        if current_l == current_r {
            return counter;
        }
    }
}

fn neighboring_pipes(grid: &Vec<Vec<char>>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let c = grid[point.1][point.0];
    match c {
        '|' => {
            vec![(point.0, point.1-1), (point.0, point.1+1)]
        },
        '-' => {
            vec![(point.0+1, point.1), (point.0-1, point.1)]
        },
        'L' => {
            vec![(point.0, point.1-1), (point.0+1, point.1)]
        },
        'J' => {
            vec![(point.0, point.1-1), (point.0-1, point.1)]
        },
        '7' => {
            vec![(point.0, point.1+1), (point.0-1, point.1)]
        },
        'F' => {
            vec![(point.0+1, point.1), (point.0, point.1+1)]
        },
        _ => panic!("Unknown char"),
    }
}

fn next_pipe(grid: &Vec<Vec<char>>, point: (usize, usize), previous: (usize, usize)) -> (usize, usize) {
    let neighbors = neighboring_pipes(grid, point);
    for n in neighbors {
        if n != previous {
            return n;
        }
    }
    panic!("Origin node provided that is not reached!");
}

/// Analyzes the surrounding pipes to determine what pipe point is
//fn determine_pipe(grid: &Vec<Vec<char>>, point: (usize, usize)) -> Option<char> {
//    let mut neighboring_pipes = Vec::new();
//    for pos in surrounding_points(point) {
//        for n in neighboring_pipes(grid, pos) {
//            
//        }
//    }
//}

fn surrounding_points(point: (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (point.0-1, point.1-1),
        (point.0-1, point.1),
        (point.0-1, point.1+1),
        (point.0, point.1-1),
        (point.0, point.1-1),
        (point.0+1, point.1-1),
        (point.0+1, point.1),
        (point.0+1, point.1+1),
    ]
}

#[cfg(test)]
mod tests {
    use crate::{neighboring_pipes, next_pipe, traverse_tunnels};


    #[test]
    fn test_neighboring_pipes() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '|', '-', 'L', 'J', '7', 'F', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(neighboring_pipes(&grid, (1, 1)), vec![(1, 0), (1, 2)]);
        assert_eq!(neighboring_pipes(&grid, (2, 1)), vec![(3, 1), (1, 1)]);
        assert_eq!(neighboring_pipes(&grid, (3, 1)), vec![(3, 0), (4, 1)]);
        assert_eq!(neighboring_pipes(&grid, (4, 1)), vec![(4, 0), (3, 1)]);
        assert_eq!(neighboring_pipes(&grid, (5, 1)), vec![(5, 2), (4, 1)]);
        assert_eq!(neighboring_pipes(&grid, (6, 1)), vec![(7, 1), (6, 2)]);
    }

    #[test]
    fn test_next_pipe() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '|', '-', 'L', 'J', '7', 'F', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(next_pipe(&grid, (1, 1), (1, 0)), (1, 2));
        assert_eq!(next_pipe(&grid, (2, 1), (3, 1)), (1, 1));
        assert_eq!(next_pipe(&grid, (3, 1), (3, 0)), (4, 1));
        assert_eq!(next_pipe(&grid, (4, 1), (4, 0)), (3, 1));
        assert_eq!(next_pipe(&grid, (5, 1), (5, 2)), (4, 1));
        assert_eq!(next_pipe(&grid, (6, 1), (7, 1)), (6, 2));
    }

    #[test]
    fn test_traverse_tunnels() {
        let mut grid = vec![
            vec!['.', 'F', '-', '-', '7', '.', '.', '.'],
            vec!['.', 'S', '.', '.', 'L', '7', '.', '.'],
            vec!['.', 'L', '-', '-', '-', 'J', '.', '.'],
        ];
        assert_eq!(traverse_tunnels(&mut grid, (1, 1), '|'), 6);
    }
}