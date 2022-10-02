use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/day15_test.txt")?; 
    let mut board: Vec<Vec<Node>> = Vec::new();
    let mut board_with_neighbors: Vec<Vec<Node>> = Vec::new();
    for (i1, line) in content.iter().enumerate() {
        let mut l: Vec<Node> = Vec::new();
        let mut l2: Vec<Node> = Vec::new();
        for (i2, c) in line.chars().enumerate() {
            l.push(Node::new((i2,i1), char::to_digit(c, 10).unwrap()));
            l2.push(Node::new((i2,i1), char::to_digit(c, 10).unwrap()));
        }
        board.push(l);
        board_with_neighbors.push(l2);
    }
    for l in &mut board_with_neighbors {
        for c in l.iter_mut() {
            for neighbors in neighbor_positions(c.pos, board.len(), board.len()) {
                c.add_neighbor(&board[neighbors.0][neighbors.1]);
            }
            print!("{}", c.risk);
        }
        println!();
    }
    Ok(())
}

pub fn part2(_debug: bool) -> Result<()> {
    Ok(())
}

fn neighbor_positions(pos: (usize, usize), max_x_size: usize, max_y_size: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    if pos.0 != 0 {
        positions.push((pos.0-1, pos.1));
    }
    if pos.1 != 0 {
        positions.push((pos.0, pos.1-1));
    }
    if pos.0 != max_x_size-1 {
        positions.push((pos.0+1, pos.1));
    }
    if pos.1 != max_y_size-1 {
        positions.push((pos.0, pos.1+1));
    }
    positions
}

fn shortest_way(start: &Node, target: &Node) -> Option<u32> {
    let mut risk = 0;
    let mut open: Vec<&Node> = Vec::new();
    let mut selected_nodes: Vec<(usize, usize)> = Vec::new();

    for neighbor in &start.neighbors {
        open.push(neighbor);
    }
    while !open.is_empty() {
        let node = open.pop().unwrap();
        if node == target {
            return Some(risk + target.risk);
        }

        for neighbor in &node.neighbors {
            let new_risk = node.risk + neighbor.risk;
            if new_risk < neighbor.risk {
                risk -= node.risk;
                risk += neighbor.risk;
                if !open.contains(neighbor) {
                    open.push(&neighbor);
                }
            }
        }
    }
    None
}

struct Node<'a> {
    pos: (usize, usize),
    risk: u32,
    neighbors: Vec<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(pos: (usize, usize), risk: u32) -> Self {
        Self {
            pos,
            risk,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, node: &'a Node) {
        self.neighbors.push(node);
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}