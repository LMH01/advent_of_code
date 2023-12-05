use adventofcode_lmh01_lib::numbers_from_string;

aoc::parts!(1, 2);

pub fn part_1(input: aoc::Input) -> impl ToString {
    let mut i = 0;
    for line in input {
        let numbers = numbers_from_string(line);
        let a0 = numbers[0];
        let a1 = numbers[1];
        let a: (u32, u32) = (a0, a1);
        let b0 = numbers[2];
        let b1 = numbers[3];
        let b: (u32, u32) = (b0, b1);
        if fully_contained(a, b) {
            i += 1;
        }
    }
    println!("To reconsider: {i}");
    i
}

pub fn part_2(input: aoc::Input) -> impl ToString {
    calculate(input.as_lines(), false)
}

fn calculate(content: &[&str], part1: bool) -> i32 {
    let mut i = 0;
    for line in content {
        let numbers = numbers_from_string(line);
        let a0 = numbers[0];
        let a1 = numbers[1];
        let a: (u32, u32) = (a0, a1);
        let b0 = numbers[2];
        let b1 = numbers[3];
        let b: (u32, u32) = (b0, b1);
        if part1 {
            if fully_contained(a, b) {
                i += 1;
            }
        } else if overlaps(a, b) {
            i += 1;
        }
    }
    println!("To reconsider: {i}");
    i
}

fn fully_contained(a: (u32, u32), b: (u32, u32)) -> bool {
    if a.0 >= b.0 && a.1 <= b.1 || b.0 >= a.0 && b.1 <= a.1 {
        return true;
    }
    false
}

fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    if a.1 >= b.0 && a.0 <= b.1 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::fully_contained;

    #[test]
    fn test_fully_contained() {
        assert!(!fully_contained((2, 4), (6, 8)));
        assert!(!fully_contained((2, 3), (4, 5)));
        assert!(!fully_contained((5, 7), (7, 9)));
        assert!(fully_contained((2, 8), (3, 7)));
        assert!(fully_contained((6, 6), (4, 6)));
        assert!(!fully_contained((2, 6), (4, 8)));
    }
}
