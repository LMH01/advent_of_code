use std::collections::HashMap;

aoc::parts!(1, 2);

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

//fn part_2(input: aoc::Input) -> impl ToString {
//    let mut steps = 0;
//    let parsed = parse(input);
//    // store the current nodes
//    let mut currents = Vec::new();
//    // find all nodes that end with 'A'
//    for entry in &parsed.1 {
//        if entry.0.ends_with('A') {
//            currents.push(*entry.0);
//        }
//    }
//    println!("Starting at {} nodes", currents.len());
//    // the start nodes have been constructed
//    'outer: loop {
//        'inner: for c in parsed.0.chars() {
//            for current in currents.iter_mut().enumerate() {
//                match c {
//                    'L' => *current.1 = parsed.1.get(current.1).unwrap().0.as_str(),
//                    'R' => *current.1 = parsed.1.get(current.1).unwrap().1.as_str(),
//                    _ => (),
//                }
//            }
//            steps += 1;
//            // check if all new currents end with 'Z'
//            let mut currents_correct = 0;
//            for current in &currents {
//                if !current.ends_with('Z') {
//                    if currents_correct > 2 {
//                        println!("{currents_correct}");
//                    }
//                    continue 'inner;
//                }
//                currents_correct += 1;
//            }
//            // when we are here we know that all current nodes end with 'Z', so we are finished
//            break 'outer;
//        }
//    }
//    steps
//}

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
