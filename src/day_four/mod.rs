use std::f32::consts::E;

use regex::{Captures, Match, Regex};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

type Direction = [(i32, i32); 4];

const DIRECTIONS: [Direction; 8] = [
    // (x, y)
    // nw
    [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    // n
    [(0, 0), (0, -1), (0, -2), (0, -3)],
    // ne
    [(0, 0), (1, -1), (2, -2), (3, -3)],
    // e
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    // se
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    // s
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    // sw
    [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
    // w
    [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
];

pub(crate) fn run() {
    println!("===== DAY THREE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let line_length = matrix.first().map(|f| f.len()).unwrap();
    let column_length = matrix.len();

    let mut total: u32 = 0;

    for y in 0..column_length {
        for x in 0..line_length {
            for direction in DIRECTIONS {
                let string = check_direction(&matrix, x, y, &direction);

                if string == "XMAS" {
                    total += 1
                }
            }
        }
    }

    total
}

fn get_matrix_char(matrix: &Vec<Vec<String>>, x: usize, y: usize) -> String {
    matrix
        .get(y)
        .and_then(|line| line.get(x))
        .cloned()
        .unwrap_or_default()
}

fn check_direction(matrix: &Vec<Vec<String>>, x: usize, y: usize, direction: &Direction) -> String {
    direction
        .iter()
        .map(|&(d_x, d_y)| {
            let new_x = (x as i32 + d_x) as usize;
            let new_y = (y as i32 + d_y) as usize;
            get_matrix_char(matrix, new_x, new_y)
        })
        .collect()
}

fn part_two(input: &str) -> u32 {
    let matrix = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let line_length = matrix.first().map(|f| f.len()).unwrap();
    let column_length = matrix.len();

    let mut total: u32 = 0;

    for y in 0..column_length - 2 {
        for x in 0..line_length - 2 {
            let tl = matrix[y][x].as_str();
            let tr = matrix[y][x + 2].as_str();
            let c = matrix[y + 1][x + 1].as_str();
            let bl = matrix[y + 2][x].as_str();
            let br = matrix[y + 2][x + 2].as_str();

            let forward_slash = format!("{tl}{c}{br}");
            let back_slash = format!("{tr}{c}{bl}");

            let mas = String::from("MAS");
            let sam = String::from("SAM");

            if (forward_slash == mas || forward_slash == sam)
                && (back_slash == mas || back_slash == sam)
            {
                total += 1
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 18;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 9;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
