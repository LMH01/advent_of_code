use std::cmp::Ordering;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines: Vec<Line> = Vec::new();
    let mut board = Board::new();

    for line in input {
        let mut p1: (&str, &str) = ("", "");
        let mut p2: (&str, &str) = ("", "");

        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((a, b)) = a.split_once(',') {
                p1 = (a, b);
            }
            if let Some((a, b)) = b.split_once(',') {
                p2 = (a, b);
            }
        }
        board.set_line_active(
            p1.1.parse().unwrap(),
            p1.0.parse().unwrap(),
            p2.1.parse().unwrap(),
            p2.0.parse().unwrap(),
            false,
        );
        lines.push(Line {
            x1: p1.1.parse().unwrap(),
            y1: p1.0.parse().unwrap(),
            x2: p2.1.parse().unwrap(),
            y2: p2.0.parse().unwrap(),
        });
    }
    println!(
        "Number of overlapping points: {}",
        board.overlapping_points()
    );
    board.overlapping_points()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut lines: Vec<Line> = Vec::new();
    let mut board = Board::new();

    for line in input {
        let mut p1: (&str, &str) = ("", "");
        let mut p2: (&str, &str) = ("", "");

        if let Some((a, b)) = line.split_once(" -> ") {
            if let Some((a, b)) = a.split_once(',') {
                p1 = (a, b);
            }
            if let Some((a, b)) = b.split_once(',') {
                p2 = (a, b);
            }
        }
        board.set_line_active(
            p1.1.parse().unwrap(),
            p1.0.parse().unwrap(),
            p2.1.parse().unwrap(),
            p2.0.parse().unwrap(),
            false,
        );
        lines.push(Line {
            x1: p1.1.parse().unwrap(),
            y1: p1.0.parse().unwrap(),
            x2: p2.1.parse().unwrap(),
            y2: p2.0.parse().unwrap(),
        });
    }

    println!(
        "Number of overlapping points: {}",
        board.overlapping_points()
    );
    board.overlapping_points()
}

#[allow(dead_code)]
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
    points: Vec<Point>,
}

impl Board {
    fn new() -> Self {
        Self {
            points: {
                let mut vec = Vec::new();
                for x in 0..=1000 {
                    for y in 0..=1000 {
                        vec.push(Point {
                            x,
                            y,
                            number_of_lines: 0,
                        });
                    }
                }
                vec
            },
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

    fn set_line_active(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        enable_diagonal: bool,
    ) -> bool {
        if x1 == x2 {
            // line is vertical
            if y1 <= y2 {
                for y in y1..=y2 {
                    self.set_point_active(x1, y);
                }
            } else {
                for y in y2..=y1 {
                    self.set_point_active(x1, y);
                }
            }
        } else if y1 == y2 {
            // line is horizontal
            if x1 <= x2 {
                for x in x1..=x2 {
                    self.set_point_active(x, y1);
                }
            } else {
                for x in x2..=x1 {
                    self.set_point_active(x, y1);
                }
            }
        } else if enable_diagonal {
            // Copy from https://mzte.de/git/LordMZTE/aoc2021/src/branch/master/src/days/day5.rs#L51
            // because i'm to stupid and tired to do it myself
            if Board::is_diagonal(x1, y1, x2, y2) {
                let xdir = ord_to_dir(x2.cmp(&x1));
                let ydir = ord_to_dir(y2.cmp(&y1));
                let mut curx = x1;
                let mut cury = y1;
                while curx.wrapping_sub(xdir as i32) != x2 && cury.wrapping_sub(ydir as i32) != y2 {
                    self.set_point_active(curx, cury);
                    //mtx[cury][curx] += 1;
                    curx += xdir as i32;
                    cury += ydir as i32;
                }
            }
        }
        true
    }

    fn is_diagonal(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let x_gap: i32 = (x1 - x2).abs();
        let y_gap: i32 = (y1 - y2).abs();
        x_gap == y_gap
    }

    fn set_point_active(&mut self, x: i32, y: i32) {
        for point in &mut self.points {
            if point.x == x && point.y == y {
                point.number_of_lines += 1;
            }
        }
    }
}

fn ord_to_dir(ord: Ordering) -> i8 {
    match ord {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
