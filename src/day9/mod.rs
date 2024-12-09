use core::num;
use std::{borrow::BorrowMut, mem, time::Instant};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY NINE =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_two(PUZZLE_INPUT);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

fn part_one(input: &str) -> u64 {
    let mut memory_map = get_memory_map(input);

    let mut i: usize = 0;
    let mut j: usize = memory_map.len() - 1;

    while i < j {
        while i < j && memory_map[i].is_some() {
            i += 1;
        }

        while j > i && memory_map[j].is_none() {
            j -= 1;
        }

        if i < j {
            memory_map.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    memory_map
        .iter()
        .enumerate()
        .filter_map(|(i, node)| node.map(|n| n * i as u64))
        .sum()
}

fn part_two(input: &str) -> u64 {
    0
}

fn get_memory_map(input: &str) -> Vec<Option<u64>> {
    input
        .chars()
        .enumerate()
        .scan(0u64, |current_id, (i, c)| {
            let file_block_size = c
                .to_string()
                .parse::<usize>()
                .expect("failed to parse file block size");

            let block_iter = if i % 2 == 0 {
                let id = *current_id;
                *current_id += 1;
                std::iter::repeat(Some(id)).take(file_block_size)
            } else {
                std::iter::repeat(None).take(file_block_size)
            };
            Some(block_iter)
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 1928;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_one_simple() {
        let expected = 60;

        let actual = super::part_one("12345");

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 2858;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
