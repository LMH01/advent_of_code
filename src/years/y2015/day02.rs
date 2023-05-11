use std::{i32::MAX, mem::discriminant, collections::BinaryHeap};

use adventofcode_lmh01_lib::read_file;
use miette::Result;

pub fn part1(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day02.txt")?;
    let mut paper = 0;
    for line in content {
        let p = Package::from_line(&line);
        paper += p.required_paper();
    }
    println!("Required paper: {paper}");
    Ok(())
}

pub fn part2(debug: bool) -> Result<()> {
    let content = read_file("input/y2015/day02.txt")?;
    let mut ribbon = 0;
    for line in content {
        let p = Package::from_line(&line);
        ribbon += p.required_ribbon();
    }
    println!("Required ribbon: {ribbon}");
    Ok(())
}

struct Package {
    l: i32,
    b: i32,
    h: i32,
}

impl Package {
    fn from_line(line: &str) -> Self {
        let dimensions: Vec<&str> = line.split("x").collect();
        Self {
            l: dimensions[0].parse().unwrap(),
            b: dimensions[1].parse().unwrap(),
            h: dimensions[2].parse().unwrap(),
        }
    }

    fn required_paper(&self) -> i32 {
        let mut vec = vec![self.l*self.b, self.l*self.h, self.b*self.h];
        vec.sort();
        vec[0]*2+vec[1]*2+vec[2]*2+vec[0]
    }

    fn required_ribbon(&self) -> i32 {
        let mut vec = vec![self.l*2+self.b*2, self.l*2+self.h*2, self.b*2+self.h*2];
        vec.sort();
        self.b*self.h*self.l+vec[0]
    }
}