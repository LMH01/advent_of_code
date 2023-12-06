use adventofcode_lmh01_lib::numbers_from_string;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let times = numbers_from_string(input[0]);
    let distances = numbers_from_string(input[1]);
    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race::new(times[i] as u64, distances[i] as u64));
    }
    let mut result = 0;
    for race in races {
        if result == 0 {
            result = race.possible_ways();
        } else {
            result *= race.possible_ways();
        }
    }
    result
}

fn part_2(input: aoc::Input) -> impl ToString {
    let time = input[0].replace("Time:", "").trim().replace(' ', "").parse::<u64>().unwrap();
    let distance = input[1].replace("Distance:", "").trim().replace(' ', "").parse::<u64>().unwrap();
    println!("{time}|{distance}");
    let race = Race::new(time, distance); 
    race.possible_ways()
}

struct Race {
    duration: u64,
    distance: u64,
}

impl Race {

    fn new(duration: u64, distance: u64) -> Self {
        Self {
            duration,
            distance,
        }
    }

    fn possible_ways(&self) -> u32 {
        let range_vec = construct_range_vec(self.duration);
        let mut possible_ways = 0;
        for x in range_vec {
            if x > self.distance {
                possible_ways += 1;
            }
        }
        possible_ways
    }
}

fn construct_range_vec(duration: u64) -> Vec<u64> {
    let mut vec = Vec::new();
    for i in 0..=duration {
        vec.push((duration-i)*i);
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::Race;


    #[test]
    fn test_possible_ways() {
        assert_eq!(Race::new(7, 9).possible_ways(), 4);
        assert_eq!(Race::new(15, 40).possible_ways(), 8);
        assert_eq!(Race::new(30, 200).possible_ways(), 9);
    }
}