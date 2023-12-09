aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines = Vec::new();
    // construct lines that we have to analyze
    for line in input {
        lines.push(numbers_from_string(line));
    }
    let mut total = 0;
    for line in lines {
        total += determine_spacing(line.as_slice());
    }
    total
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

/// Return value is the last value of the line
fn determine_spacing(line: &[i64]) -> i64 {
    let mut zero = true;
    let mut last_element = None;
    let mut new_line = Vec::new();
    for x in line {
        if *x != 0 {
            zero = false;
        }
        match last_element {
            None => last_element = Some(*x),
            Some(last) => {
                // update new line
                new_line.push(x - last);
                last_element = Some(*x);
            }
        }
    }
    // check if zero line was found, if yes we can begin to traverse back up
    if zero {
        return 0;
    }
    determine_spacing(&new_line.as_slice()) + line.last().unwrap_or(&0)
}

fn numbers_from_string(input: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    let chunks = input.split(' ').collect::<Vec<&str>>();
    for chunk in chunks {
        numbers.push(chunk.parse().unwrap())
    }
    numbers
}

#[cfg(test)]
mod tests {
    use crate::{determine_spacing, numbers_from_string};

    #[test]
    fn test_determine_spacing() {
        assert_eq!(determine_spacing(vec![].as_slice()), 0);
        assert_eq!(determine_spacing(vec![0, 3, 6, 9, 12, 15].as_slice()), 18);
        assert_eq!(determine_spacing(vec![1, 3, 6, 10, 15, 21].as_slice()), 28);
        assert_eq!(
            determine_spacing(vec![10, 13, 16, 21, 30, 45].as_slice()),
            68
        );
        assert_eq!(determine_spacing(vec![-3, -3, -3, -3].as_slice()), -3);
        assert_eq!(determine_spacing(vec![3, 2, 1].as_slice()), 0);
        assert_eq!(determine_spacing(vec![0, 2].as_slice()), 4);
        assert_eq!(determine_spacing(vec![12, 9, 6, 3, 0, -3, -6, -9, -12, -15, -18, -21, -24, -27, -30, -33, -36, -39, -42, -45, -48].as_slice()), -51);
    }

    #[test]
    fn test_numbers_from_string() {
        assert_eq!(numbers_from_string("-3 -5 -10"), vec![-3, -5, -10])
    }
}
