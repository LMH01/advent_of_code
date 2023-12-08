use std::{
    collections::HashMap,
    sync::{mpsc::{self, Receiver, Sender}, RwLock, Arc},
    thread, process::exit,
};

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
//    launch_threads(parsed.0, parsed.1)
//}
//
///// Launches as many threads as there are nodes that end with 'A'.
///// Threads report to control thread when 'Z' is last char in string and the amount of steps
///// it took to reach. Control thread will terminate when a timestamp contains 6 entries.
///// This is the number of steps it took.
//fn launch_threads(directions: String, nodes: HashMap<String, (String, String)>) -> u32 {
//    // channel to transmit result
//    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();
//
//    let mut start_number = 0;
//    for entry in &nodes {
//        if entry.0.ends_with('A') {
//            start_number += 1;
//        }
//    }
//
//    // determine the start nodes
//    let mut start_nodes = Vec::new();
//    for entry in &nodes {
//        if entry.0.ends_with('A') {
//            start_nodes.push(entry.0.to_string());
//        }
//    }
//
//    println!("Dispatching {start_number} threads to calculate paths");
//    for i in 0..start_number {
//        // Clone required variables
//        let tx = tx.clone();
//        let directions = directions.clone();
//        let nodes = nodes.clone();
//        let start_node = start_nodes[i].clone();
//
//        // spawn thread
//        thread::spawn(move || {
//            println!("Thread [{i}]: Starting travel from node {start_node}");
//            let mut current = start_node.as_str();
//            let mut steps = 0;
//            loop {
//                for c in directions.chars() {
//                    match c {
//                        'L' => current = nodes.get(current).unwrap().0.as_str(),
//                        'R' => current = nodes.get(current).unwrap().1.as_str(),
//                        _ => (),
//                    }
//                    steps += 1;
//                    if current.ends_with('Z') {
//                        // send to control thread that a step was found that contains 'Z'
//                        // as last char
//                        if let Err(_) = tx.send(steps) {
//                            // if we can't send we know that the control thread has died,
//                            // thus this tread can stop too
//                            println!("Thread [{i}]: terminating");
//                            exit(0);
//                        }
//                    }
//                }
//            }
//        });
//    }
//
//    let mut received_steps = HashMap::new();
//    loop {
//        match rx.recv() {
//            Ok(v) => {
//                if let Some(times) = received_steps.get_mut(&v) {
//                    *times += 1;
//                    if *times == 5 {
//                        println!("Occurred 5 times: {v}");
//                    }
//                    if *times == start_number {
//                        println!("[{start_number}] times found: {v}");
//                        return v;
//                    }
//                } else {
//                    received_steps.insert(v, 1);
//                }
//            }
//            Err(_) => break,
//        }
//    }
//    0
//}

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
fn parse(input: aoc::Input) -> (String, HashMap<String, (String, String)>) {
    let mut navigation = None;
    let mut map = HashMap::new();
    for (idx, line) in input.as_lines().iter().enumerate() {
        if idx == 0 {
            navigation = Some(String::from(*line));
        } else if idx >= 2 {
            let chunks = line.split(' ').collect::<Vec<&str>>();
            let key = chunks[0].to_string();
            let value = (
                chunks[2].replace('(', "").replace(',', ""),
                chunks[3].replace(')', ""),
            );
            map.insert(key, value);
        }
    }
    (navigation.unwrap(), map)
}
