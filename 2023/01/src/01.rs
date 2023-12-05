aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let vec = create_vec(input.as_lines());
    let total = get_total(&vec);
    println!("Total calibration value: {total}");
    total
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let mut vec = create_vec(input.as_lines());
    replace_string_nr(&mut vec);
    let total = get_total(&vec);
    println!("Total calibration value: {total}");
    total
}

pub fn replace_string_nr(content: &mut Vec<String>) {
    for line in content {
        *line = line.replace("one", "o1e");
        *line = line.replace("two", "t2o");
        *line = line.replace("three", "t3r");
        *line = line.replace("four", "f4r");
        *line = line.replace("five", "f5e");
        *line = line.replace("six", "s6x");
        *line = line.replace("seven", "s7n");
        *line = line.replace("eight", "e8t");
        *line = line.replace("nine", "n9e");
    }
}

fn get_total(content: &Vec<String>) -> i32 {
    let mut total = 0;
    for line in content {
        let mut result = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                result.push(c);
                break;
            }
        }
        total += result.parse().unwrap_or(0);
    }
    total
}

fn create_vec(input: &[&str]) -> Vec<String> {
    let mut vec = Vec::new();
    for line in input {
        vec.push(line.to_string());
    }
    vec
}
