use adventofcode_lmh01_lib::read_file;

use std::{collections::HashMap, error::Error};
use std::io::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let vec = read_file("input.txt")?;
    let mut lines: Vec<Line> = Vec::new();
    let mut board = Board::new();

    for line in vec {
        let mut p1: (&str, &str) = ("", "");
        let mut p2: (&str, &str) = ("", "");

        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((a, b)) = a.split_once(",") {
                p1 = (a, b);
            }
            if let Some((a, b)) = b.split_once(",") {
                p2 = (a, b);
            }
        }
        println!("Setting line active: ({}, {}) - ({}, {})", p1.1, p1.0, p2.1, p2.0);
        board.set_line_active(p1.1.parse()?, p1.0.parse()?, p2.1.parse()?, p2.0.parse()?);
        lines.push(Line{x1: p1.1.parse()?, y1: p1.0.parse()?, x2: p2.1.parse()?, y2: p2.0.parse()?});
    }

    for line in lines {
        println!("({}, {}), ({}, {})", line.y1, line.x1, line.y2, line.x2);
    }

    board.print_board();
    println!("Number of overlapping points: {}", board.overlapping_points());
    Ok(())
}

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

struct Point {
    x: i32,
    y: i32,
    number_of_lines: i32,
}

struct Board {
    points: Vec<Point>
}

impl Board {
    fn new() -> Self {
        Self {
            points: {
                let mut vec = Vec::new();
                for x in 0..=1000 {
                    for y in 0..=1000 {
                        vec.push(Point{x, y, number_of_lines: 0});
                    }
                }
                vec
            }
        }
    }

    fn overlapping_points(&self) -> i32 {
        let mut number = 0;
        for point in &self.points {
            if point.number_of_lines >= 2 {
                number += 1;
            }
        }
        number
    }

    fn set_line_active(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        if x1 == x2 {// line is vertical
            if y1<=y2 {
                for y in y1..=y2 {
                    &self.set_point_active(x1, y);
                }
            } else {
                for y in y2..=y1 {
                    &self.set_point_active(x1, y);
                }
            }
        } else if y1 == y2 {// line is horizontal
            if x1<=x2 {
                for x in x1..=x2 {
                    &self.set_point_active(x, y1);
                }
            } else {
                for x in x2..=x1 {
                    &self.set_point_active(x, y1);
                }
            }
        } else {
            return false;
        }
        true
    }

    fn set_point_active(&mut self, x: i32, y: i32) {
        for point in &mut self.points {
            if point.x == x && point.y == y {
                //println!("Increased number of lines for point ({}, {})", x, y);
                point.number_of_lines += 1;
            }
        }
    }

    fn print_board(&self) {
        for x in 0..=1000 {
            for y in 0..=1000 {
                for point in &self.points {
                    if point.y == y && point.x == x {
                        if point.number_of_lines == 0 {
                            print!(".");
                        } else if point.number_of_lines == 1 {
                            print!("1");
                        } else if point.number_of_lines >= 2 {
                            print!("{}", point.number_of_lines);
                        }
                    }
                }
            }
            println!();
        }
    }
}