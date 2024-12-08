use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    ops::{Add, Mul},
    time::Instant,
};

use crate::models::{bounding_box::BoundingBox, point::Point};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY EIGHT =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_two(PUZZLE_INPUT);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

fn part_one(input: &str) -> usize {
    // get height and width of the matrix
    let width: usize = input.find('\n').unwrap() as usize;
    let height: usize = (input.len()) as usize / width;

    let sanitized = input.replace('\n', "");

    // create a bounding box which goes from (0, 0) to (width -1, height -1)
    let bounding_box = BoundingBox::new(Point::from((0, 0)), Point::from((width - 1, height - 1)));

    /*
     * Set up some hash sets and maps.
     * - One set for the position of all antennas
     * - One map for the buckets of antennas + types
     * - one set for the position of all anti-nodes
     */

    // Buckets for those antennas, so we can check _only_ antennas which would produce an anti-node
    let mut antennas: HashMap<char, HashSet<Point>> = HashMap::new();

    // Positions of all anti-nodes
    let mut antinode_positions: HashSet<Point> = HashSet::new();

    //parse all characters into their resepective buckets

    for (i, c) in sanitized.chars().enumerate() {
        if c.is_alphanumeric() {
            let col: usize = i as usize / width;
            let row: usize = i as usize % width;

            antennas
                .entry(c)
                .or_insert_with(HashSet::new)
                .insert(Point::from((row, col)));
        }
    }

    // for each set of antennas for each antenna type
    for coords in antennas.values() {
        for (i, a) in coords.iter().enumerate() {
            for b in coords.iter().skip(i + 1) {
                let d = if a < b { *b - *a } else { *a - *b };

                let (left_point, right_point) = if a < b {
                    (*a - d, *b + d)
                } else {
                    (*b - d, *a + d)
                };

                // handle left first
                if bounding_box.contains(left_point) {
                    antinode_positions.insert(left_point);
                }

                if bounding_box.contains(right_point) {
                    antinode_positions.insert(right_point);
                }
            }
        }
    }

    antinode_positions.len()
}

fn part_two(input: &str) -> usize {
    // get height and width of the matrix
    let width: usize = input.find('\n').unwrap() as usize;
    let height: usize = (input.len()) as usize / width;

    let sanitized = input.replace('\n', "");

    // create a bounding box which goes from (0, 0) to (width -1, height -1)
    let bounding_box = BoundingBox::new(Point::from((0, 0)), Point::from((width - 1, height - 1)));

    /*
     * Set up some hash sets and maps.
     * - One set for the position of all antennas
     * - One map for the buckets of antennas + types
     * - one set for the position of all anti-nodes
     */

    // Buckets for those antennas, so we can check _only_ antennas which would produce an anti-node
    let mut antennas: HashMap<char, HashSet<Point>> = HashMap::new();

    // Positions of all anti-nodes
    let mut antinode_positions: HashSet<Point> = HashSet::new();

    //parse all characters into their resepective buckets

    for (i, c) in sanitized.chars().enumerate() {
        if c.is_alphanumeric() {
            let col: usize = i as usize / width;
            let row: usize = i as usize % width;

            antennas
                .entry(c)
                .or_insert_with(HashSet::new)
                .insert(Point::from((row, col)));
        }
    }

    // for each set of antennas for each antenna type
    for coords in antennas.values() {
        for (i, a) in coords.iter().enumerate() {
            for b in coords.iter().skip(i + 1) {
                let d = if a < b { *b - *a } else { *a - *b };

                let (mut upper, mut lower) = if a < b { (*a, *b) } else { (*b, *a) };

                while bounding_box.contains(upper) {
                    antinode_positions.insert(upper);
                    upper -= d;
                }

                while bounding_box.contains(lower) {
                    antinode_positions.insert(lower);
                    lower += d;
                }
            }
        }
    }

    antinode_positions.len()
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 14;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 34;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_one_two_antennas() {
        let expected = 2;

        let actual = super::part_one(
            r"..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........",
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_one_with_3_antennas() {
        let expected = 4;

        let actual = super::part_one(
            r"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_one_with_4_antennas_with_1_different() {
        let expected = 4;

        let actual = super::part_one(
            r"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........",
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_with_3_antennas() {
        let expected = 9;

        let actual = super::part_two(
            r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        );

        assert_eq!(actual, expected);
    }
}
