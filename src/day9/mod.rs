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
    let mut memory_map = get_memory_map(input);

    let blocks = get_blocks_from_memory_map(&memory_map);

    let mut empty_blocks = blocks
        .iter()
        .filter(|block| block.id().is_none())
        .copied()
        .collect::<Vec<FileBlock>>();
    let filled_blocks = blocks
        .iter()
        .filter(|block| block.id().is_some())
        .rev()
        .copied()
        .collect::<Vec<FileBlock>>();

    for filled_block in filled_blocks {
        for empty_block in &mut empty_blocks {
            if empty_block.size() >= filled_block.size()
                && empty_block.start() < filled_block.start()
            {
                for j in 0..filled_block.size() {
                    memory_map[empty_block.start() + j] = filled_block.id();
                    memory_map[filled_block.start() + j] = None;
                }

                empty_block.decrease_size(filled_block.size());
                break;
            }
        }
    }

    memory_map
        .iter()
        .enumerate()
        .filter_map(|(i, node)| node.map(|n| n * i as u64))
        .sum()
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

#[derive(Debug, Copy, Clone)]
struct FileBlock(pub Option<u64>, pub usize, pub usize);

impl FileBlock {
    pub fn new(id: Option<u64>, start_index: usize, size: usize) -> Self {
        Self(id, start_index, size)
    }

    pub fn id(&self) -> Option<u64> {
        self.0
    }

    pub fn start(&self) -> usize {
        self.1
    }

    pub fn size(&self) -> usize {
        self.2
    }

    pub fn increase_size(&self) -> Self {
        Self(self.id(), self.start(), self.size() + 1)
    }

    pub fn decrease_size(&mut self, n: usize) {
        self.1 += n;
        self.2 -= n;
    }
}

fn get_blocks_from_memory_map(memory_map: &Vec<Option<u64>>) -> Vec<FileBlock> {
    let mut file_blocks: Vec<FileBlock> = Vec::new();

    let mut file_block: Option<FileBlock> = None;

    for (i, value) in memory_map.iter().enumerate() {
        file_block = match file_block {
            Some(fb) => {
                if *value == fb.id() {
                    Some(fb.increase_size())
                } else {
                    file_blocks.push(fb);
                    Some(FileBlock::new(*value, i, 1))
                }
            }
            None => Some(FileBlock::new(*value, i, 1)),
        }
    }

    file_blocks.push(file_block.unwrap());

    file_blocks
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
