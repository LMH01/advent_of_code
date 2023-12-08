use std::collections::HashMap;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut steps = 0;
    let parsed = parse(input);
    let mut current = "AAA";
    for entry in &parsed.1 {
        println!("{}-{:?}", entry.0, entry.1);
    }
    loop {
        for c in parsed.0.chars() {
            match c {
                'L' => current = parsed.1.get(current).unwrap().0.as_str(),
                'R' => current = parsed.1.get(current).unwrap().1.as_str(),
                _ => (),
            }
            steps += 1;
            if current == "ZZZ" {
                break;
            }
        }
        if current == "ZZZ" {
            break;
        }
    }
    steps
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

/// Tuple.0 = navigation
/// Tuple.1 = constructed hash map
fn parse(input: aoc::Input) -> (String, HashMap<&str, (String, String)>) {
    let mut navigation = None;
    let mut map = HashMap::new();
    for (idx, line) in input.as_lines().iter().enumerate() {
        if idx == 0 {
            navigation = Some(String::from(*line));
        } else if idx >= 2 {
            let chunks = line.split(' ').collect::<Vec<&str>>();
            let key = chunks[0];
            let value = (
                chunks[2].replace('(', "").replace(',', ""),
                chunks[3].replace(')', ""),
            );
            map.insert(key, value);
        }
    }
    (navigation.unwrap(), map)
}
