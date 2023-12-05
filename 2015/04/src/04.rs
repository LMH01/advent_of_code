use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::{self, available_parallelism},
};

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let input = input[0];
    let cores = available_parallelism().unwrap().get();
    let number = launch_threads(cores, input, "00000");
    println!("Number found: {number}");
    number
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    let input = input[0];
    let cores = available_parallelism().unwrap().get();
    let number = launch_threads(cores, input, "000000");
    println!("Number found: {number}");
    number
}

/// Launches `amount` number of threads that will simultaneously try to brute-force the hash.
///
/// Returns the number that is concatenated to `prefix` to receive a hash that starts with `delimiter`.
fn launch_threads(amount: usize, prefix: &str, delimiter: &str) -> i32 {
    let shared = Arc::new(Shared::new());
    let prefix = Arc::new(String::from(prefix));
    let delimiter = Arc::new(String::from(delimiter));

    // channel to transmit result
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    println!("Dispatching {amount} threads to brute-force hash");
    for _i in 1..=amount {
        // Clone required variables
        let shared = shared.clone();
        let prefix = prefix.clone();
        let delimiter = delimiter.clone();
        let tx = tx.clone();

        // spawn thread
        thread::spawn(move || loop {
            // check if number has been found and thread can be stopped
            if *shared.done.read().unwrap() {
                break;
            }

            // Acquire number that should be checked and increase value
            let mut current = shared.next.lock().unwrap();
            let num = *current;
            *current += 1;

            // make mutex available for all threads again
            drop(current);

            // Setup and compute hash
            let mut to_hash = (*prefix).clone();
            to_hash.push_str(&num.to_string());
            if hash_and_validate(&to_hash, &(*delimiter).clone()) {
                // Set computation as done
                *shared.done.write().unwrap() = true;

                // Result can be ignored because thread will shut down either way
                let _x = tx.send(num);
                break;
            }
        });
    }

    rx.recv().unwrap()
}

struct Shared {
    next: Mutex<i32>,
    /// Stores if computation is finished, used to terminate remaining threads
    done: RwLock<bool>,
}

impl Shared {
    fn new() -> Self {
        Self {
            next: Mutex::new(0),
            done: RwLock::new(false),
        }
    }
}

/// Returns true when the input produces a md5 hash that starts with delimiter
fn hash_and_validate(input: &str, delimiter: &str) -> bool {
    let res = md5::compute(input);
    format!("{:?}", res).starts_with(delimiter)
}
