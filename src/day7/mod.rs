use rayon::prelude::*;
use std::time::Instant;

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY SEVEN =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_two(PUZZLE_INPUT);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

fn part_one(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let mut parsed = parse_line(line);

            let values = parsed.split_off(1);

            let result = parsed.get(0).unwrap();

            if evaluate(result, &values, 2) {
                return Some(*result);
            } else {
                return None;
            }
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input
        .par_lines()
        .filter_map(|line| {
            let mut parsed = parse_line(line);

            let values = parsed.split_off(1);

            let result = parsed.get(0).unwrap();

            if evaluate(result, &values, 3) {
                return Some(*result);
            } else {
                return None;
            }
        })
        .sum()
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split(": ")
        .flat_map(|part| {
            part.split_whitespace()
                .filter_map(|value| value.parse::<u64>().ok())
        })
        .collect()
}

fn evaluate(result: &u64, values: &Vec<u64>, num_operations: u64) -> bool {
    let n = values.len();

    let number_of_combinations = num_operations.pow(n as u32 - 1);

    for i in 0..number_of_combinations {
        let mut out = values[0];

        let mut temp = i;

        for j in 0..n - 1 {
            let op_index = temp % num_operations;

            temp /= num_operations;

            out = do_operation(op_index, out, values[j + 1])
        }

        if out == *result {
            return true;
        }
    }

    return false;
}

fn do_operation(index: u64, lhs: u64, rhs: u64) -> u64 {
    match index {
        0 => lhs + rhs,
        1 => lhs * rhs,
        2 => combine(lhs, rhs),
        _ => unimplemented!(),
    }
}

fn combine(lhs: u64, rhs: u64) -> u64 {
    let mut rhs_digits: u32 = 0;
    let mut temp = rhs;

    while temp > 0 {
        rhs_digits += 1;
        temp /= 10;
    }

    lhs * 10u64.pow(rhs_digits) + rhs
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 3749;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 11387;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
