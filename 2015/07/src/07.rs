use std::collections::{HashMap, LinkedList};

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut instructions = Vec::new();
    for line in input {
        instructions.push(Instruction::from_str(line).unwrap());
    }
    // Load values
    let mut available_values = HashMap::new();
    let mut open_instructions = LinkedList::new();
    load_values(&instructions, &mut available_values, &mut open_instructions);

    // Update available values for each open instruction
    update_values(&mut open_instructions, &mut available_values, None);

    println!(
        "Value of a: {}",
        available_values.get("a").expect("Not found!")
    );
    *available_values.get("a").expect("not found")
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut instructions = Vec::new();
    for line in input {
        instructions.push(Instruction::from_str(line).unwrap());
    }
    // Load values
    let mut available_values = HashMap::new();
    let mut open_instructions = LinkedList::new();
    load_values(&instructions, &mut available_values, &mut open_instructions);

    // Update available values for each open instruction
    update_values(&mut open_instructions, &mut available_values, None);

    // Recalculate everything for different b
    let b = *available_values.get("a").expect("Not found!");
    // Clear existing values
    available_values.clear();
    open_instructions.clear();
    load_values(&instructions, &mut available_values, &mut open_instructions);
    update_values(&mut open_instructions, &mut available_values, Some(b));

    println!(
        "Value of a: {}",
        available_values.get("a").expect("Not found!")
    );
    *available_values.get("a").expect("not found")
}

/// Loads already existing values
fn load_values(
    instructions: &Vec<Instruction>,
    available_values: &mut HashMap<String, u16>,
    open_instructions: &mut LinkedList<Instruction>,
) {
    for instruction in instructions {
        match instruction {
            Instruction::LoadValue(v, k) => {
                available_values.insert(k.clone(), *v);
            }
            _ => open_instructions.push_back(instruction.clone()),
        };
    }
}

/// If `forced_value` is set, the value contained in the option will be set value for b on each loop
fn update_values(
    open_instructions: &mut LinkedList<Instruction>,
    available_values: &mut HashMap<String, u16>,
    forced_value: Option<u16>,
) {
    while !open_instructions.is_empty() {
        if let Some(value) = forced_value {
            // not pretty but it works
            available_values.insert(String::from("b"), value);
        }
        let current = open_instructions.pop_front().unwrap();
        match current.clone() {
            Instruction::And(x, y, z) => {
                if let Some(v) = get_value(Operation::And, &x, &y, available_values) {
                    available_values.insert(z, v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            Instruction::Or(x, y, z) => {
                if let Some(v) = get_value(Operation::Or, &x, &y, available_values) {
                    available_values.insert(z, v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            Instruction::LShift(x, y, z) => {
                if available_values.contains_key(&x) {
                    let v = available_values.get(&x).unwrap() << y;
                    available_values.insert(z, v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            Instruction::RShift(x, y, z) => {
                if available_values.contains_key(&x) {
                    let v = available_values.get(&x).unwrap() >> y;
                    available_values.insert(z, v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            Instruction::Not(x, y) => {
                if available_values.contains_key(&x) {
                    let v = !available_values.get(&x).unwrap();
                    available_values.insert(y, v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            Instruction::Redirect(x, y) => {
                if available_values.contains_key(&x) {
                    let v = available_values.get(&x).unwrap();
                    available_values.insert(y, *v);
                } else {
                    open_instructions.push_back(current);
                }
            }
            _ => (),
        }
    }
}

/// Tries to parse a new value:
///
/// Checks if `v1` and `v2`s values are contained in `available_values`.
/// If values are contained, these values will be used to calculate the new value.
/// If the values are not contained, it will be tried to parse them into u16.
/// If that succeeds, these values are used to calculate the new value.
/// If nothing works, `None` is returned.
fn get_value(
    operation: Operation,
    v1: &str,
    v2: &str,
    available_values: &HashMap<String, u16>,
) -> Option<u16> {
    if v1.parse::<u16>().is_ok() && v2.parse::<u16>().is_ok() {
        return Some(operation.calculate(&v1.parse::<u16>().unwrap(), &v2.parse::<u16>().unwrap()));
    } else if v1.parse::<u16>().is_ok() && available_values.contains_key(v2) {
        return Some(operation.calculate(
            &v1.parse::<u16>().unwrap(),
            available_values.get(v2).unwrap(),
        ));
    } else if available_values.contains_key(v1) && v2.parse::<u16>().is_ok() {
        return Some(operation.calculate(
            available_values.get(v1).unwrap(),
            &v1.parse::<u16>().unwrap(),
        ));
    } else if available_values.contains_key(v1) && available_values.contains_key(v2) {
        return Some(operation.calculate(
            available_values.get(v1).unwrap(),
            available_values.get(v2).unwrap(),
        ));
    }
    None
}

enum Operation {
    And,
    Or,
}

impl Operation {
    fn calculate(&self, v1: &u16, v2: &u16) -> u16 {
        match self {
            Operation::And => v1 & v2,
            Operation::Or => v1 | v2,
        }
    }
}

#[derive(Clone)]
enum Instruction {
    /// 123 -> x
    LoadValue(u16, String),
    /// x -> y
    Redirect(String, String),
    /// x AND y -> z
    And(String, String, String),
    /// x OR y -> Z
    Or(String, String, String),
    /// x LSHIFT 2 -> y
    LShift(String, u8, String),
    /// x RSHIFT 2 -> y
    RShift(String, u8, String),
    /// NOT x -> y
    Not(String, String),
}

impl Instruction {
    fn from_str(string: &str) -> Result<Instruction, String> {
        let splits: Vec<&str> = string.split(' ').collect();
        if splits.len() == 3 {
            // Instruction is load
            let num = splits[0].parse();
            if num.is_err() {
                return Ok(Instruction::Redirect(
                    String::from(splits[0]),
                    String::from(splits[2]),
                ));
            }
            return Ok(Instruction::LoadValue(
                num.unwrap(),
                String::from(splits[2]),
            ));
        } else if splits.len() == 4 {
            // instruction is NOT
            return Ok(Instruction::Not(
                String::from(splits[1]),
                String::from(splits[3]),
            ));
        } else if splits.len() == 5 {
            match splits[1] {
                "AND" => {
                    return Ok(Instruction::And(
                        String::from(splits[0]),
                        String::from(splits[2]),
                        String::from(splits[4]),
                    ))
                }
                "OR" => {
                    return Ok(Instruction::Or(
                        String::from(splits[0]),
                        String::from(splits[2]),
                        String::from(splits[4]),
                    ))
                }
                "LSHIFT" => {
                    let num = splits[2].parse();
                    if num.is_err() {
                        return Err(format!("Unable to parse input value: {}", num.unwrap_err()));
                    }
                    return Ok(Instruction::LShift(
                        String::from(splits[0]),
                        num.unwrap(),
                        String::from(splits[4]),
                    ));
                }
                "RSHIFT" => {
                    let num = splits[2].parse();
                    if num.is_err() {
                        return Err(format!("Unable to parse input value: {}", num.unwrap_err()));
                    }
                    return Ok(Instruction::RShift(
                        String::from(splits[0]),
                        num.unwrap(),
                        String::from(splits[4]),
                    ));
                }
                _ => {
                    return Err(format!(
                        "Unable to parse instruction: Unknown operation: {}",
                        splits[1]
                    ))
                }
            }
        }
        Err(format!(
            "Unable to parse string, unexpected input length!: {} - {}",
            string,
            splits.len()
        ))
    }
}
