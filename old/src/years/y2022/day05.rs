use adventofcode_lmh01_lib::read_file_absolute;
use miette::Result;

pub fn part_1(input: aoc::Input) -> impl ToString {
    let content = read_file_absolute("input/y2022/day05.txt")?;
    let mut container = TowerContainer::from_input(&content);
    let tower_height = container.max_height;
    for (index, line) in content.iter().enumerate() {
        if index > (tower_height + 1).try_into().unwrap() {
            let instructions = parse_instructions(line);
            container.move_crate(instructions.0, instructions.1, instructions.2);
        }
    }
    println!("\nMessage: {}", container.message());
    Ok(())
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let content = read_file_absolute("input/y2022/day05.txt")?;
    let mut container = TowerContainer::from_input(&content);
    let tower_height = container.max_height;
    for (index, line) in content.iter().enumerate() {
        if index > (tower_height + 1).try_into().unwrap() {
            let instructions = parse_instructions(line);
            container.move_crate_part2(instructions.0, instructions.1, instructions.2);
        }
    }
    println!("\nMessage: {}", container.message());
    Ok(())
}

struct TowerContainer {
    tower: Vec<Vec<char>>,
    max_height: u32,
}

impl TowerContainer {
    /// Creates a tower container from the puzzle input
    pub fn from_input(content: &Vec<String>) -> Self {
        let mut tower: Vec<Vec<char>> = Vec::new();
        let tower_stats = tower_stats(content);
        println!("Tower stats: {}, {}", tower_stats.0, tower_stats.1);
        for i in 1..=tower_stats.0 {
            tower.push(parse_tower(content, tower_stats.1, i));
        }
        let container = Self {
            tower,
            max_height: tower_stats.1,
        };
        container.print_tower();
        container
    }

    /// Prints the towers that are contained in this container
    pub fn print_tower(&self) {
        for (i, v) in self.tower.iter().enumerate() {
            print!("Tower {}: ", i);
            for c in v {
                print!("{}", c);
            }
            println!();
        }
    }

    /// Moves the specified amount of crates from origin to target.
    pub fn move_crate(&mut self, crate_amount: u32, origin: u32, target: u32) {
        let origin_index: usize = (origin - 1).try_into().unwrap();
        let target_index: usize = (target - 1).try_into().unwrap();
        for _i in 1..=crate_amount {
            let c = self.tower.get_mut(origin_index).unwrap().pop().unwrap();
            self.tower.get_mut(target_index).unwrap().push(c);
        }
    }

    /// Moves the specified amount of crates from origin to target. Retains order of elements.
    pub fn move_crate_part2(&mut self, crate_amount: u32, origin: u32, target: u32) {
        let mut tmp: Vec<char> = Vec::new();
        let origin_index: usize = (origin - 1).try_into().unwrap();
        let target_index: usize = (target - 1).try_into().unwrap();
        for _i in 1..=crate_amount {
            let c = self.tower.get_mut(origin_index).unwrap().pop().unwrap();
            tmp.push(c);
        }
        for _i in 1..=crate_amount {
            self.tower
                .get_mut(target_index)
                .unwrap()
                .push(tmp.pop().unwrap());
        }
    }

    /// Creates a message from the top letters on each tower.
    pub fn message(&self) -> String {
        let mut message = String::new();
        for t in &self.tower {
            message.push(t.clone().pop().unwrap());
        }
        message
    }
}

/// Returns tuple that contains the number of towers (0) and the maximum height (1).
fn tower_stats(content: &Vec<String>) -> (u32, u32) {
    let mut tower_number = 0;
    let mut tower_line: u32 = 0;
    for line in content {
        if line.is_empty() {
            break;
        }
        tower_line += 1;
    }
    // Line with tower numbers is found
    let line = content.get((tower_line - 1) as usize).unwrap().clone();
    for c in line.chars() {
        if c.is_ascii_digit() {
            let number = c.to_digit(10).unwrap();
            if tower_number < number {
                tower_number = number;
            }
        }
    }
    (tower_number, tower_line - 1)
}

/// Parses a single tower
/// # Arguemnts
/// `max_height` The maximum height of the tower
/// `tower_number` The number of the current tower
fn parse_tower(content: &[String], max_height: u32, tower_number: u32) -> Vec<char> {
    let mut tower: Vec<char> = Vec::new();
    let char_offset: usize = (1 + (tower_number - 1) * 4).try_into().unwrap();
    for i in (0..max_height).rev() {
        if let Some(c) = content.get(i as usize).unwrap().chars().nth(char_offset) {
            if !c.is_ascii_alphabetic() {
                break;
            }
            tower.push(c);
        }
    }
    tower
}

fn parse_instructions(line: &str) -> (u32, u32, u32) {
    let mut current_number = String::new();
    let mut crate_amount: u32 = 0;
    let mut origin: u32 = 0;
    let mut current_data = 0;
    for c in line.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else if !current_number.is_empty() {
            match current_data {
                0 => crate_amount = current_number.parse().unwrap(),
                1 => origin = current_number.parse().unwrap(),
                _ => (),
            };
            current_number = String::new();
            current_data += 1;
        }
    }
    let target = current_number.parse().unwrap();
    (crate_amount, origin, target)
}
