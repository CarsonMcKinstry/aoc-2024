use std::{collections::HashSet, time::Instant};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY ELEVEN =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT, 25);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_one(PUZZLE_INPUT, 75);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

fn part_one(input: &str, n_blinks: usize) -> usize {
    input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().expect("failed to parse stone"))
        .flat_map(|stone| mutate_stone(&stone, n_blinks, 0))
        .count()
}

fn part_two(input: &str) -> usize {
    0
}

fn count_digits(n: &u64) -> u32 {
    let mut n_digits: u32 = 0;
    let mut temp = *n;

    while temp > 0 {
        n_digits += 1;
        temp /= 10
    }

    n_digits
}

fn mutate_stone(stone: &u64, n_blinks: usize, current_blinks: usize) -> Vec<u64> {
    if n_blinks == current_blinks {
        return vec![*stone];
    }

    let num_digits = count_digits(&stone);

    let next_stones = if stone == &0 {
        vec![1]
    } else if num_digits % 2 == 0 {
        let split_index = num_digits / 2;
        let factor = 10u64.pow(split_index);

        let left = stone / factor;
        let right = stone % factor;

        vec![left, right]
    } else {
        vec![stone * 2024]
    };

    next_stones
        .iter()
        .flat_map(|stone| {
            let next = mutate_stone(stone, n_blinks, current_blinks + 1);
            return next;
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    mod part_one {
        use super::EXAMPLE_INPUT;

        #[test]
        fn part_one_example_returns_the_correct_answer() {
            let expected = 55312;

            let actual = super::part_one(EXAMPLE_INPUT, 25);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_one_simple() {
            let expected = 22;

            let actual = super::part_one(EXAMPLE_INPUT, 6);

            assert_eq!(actual, expected);
        }
    }

    mod mutate_stones {
        #[test]
        fn mutate_stone_with_0_once() {
            let expected = vec![1];

            let actual = super::super::mutate_stone(&0, 1, 0);

            assert_eq!(expected, actual);
        }

        #[test]
        fn mutate_stone_with_0_twice() {
            let expected = vec![2024];

            let actual = super::super::mutate_stone(&0, 2, 0);

            assert_eq!(expected, actual);
        }

        #[test]
        fn mutate_stone_with_0_thrice() {
            let expected = vec![20, 24];

            let actual = super::super::mutate_stone(&0, 3, 0);

            assert_eq!(expected, actual);
        }
    }
}
