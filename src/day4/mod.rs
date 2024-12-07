use crate::models::matrix::Matrix;

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

type Direction = (i32, i32);

const DIRECTIONS: [Direction; 8] = [
    // (x, y)
    // nw
    (-1, -1),
    // n
    (0, -1),
    // ne
    (1, -1),
    // e
    (1, 0),
    // se
    (1, 1),
    // s
    (0, 1),
    // sw
    (-1, 1),
    // w
    (-1, 0),
];

pub(crate) fn run() {
    println!("===== DAY FOUR =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    let matrix = Matrix::from(input);

    let mut total: u32 = 0;

    for (x, y) in matrix.pairs() {
        for direction in DIRECTIONS {
            let string = check_direction(&matrix, x, y, &direction, 3);
            if string == "XMAS" {
                total += 1
            }
        }
    }

    total
}

fn check_direction(
    matrix: &Matrix<String>,
    x: usize,
    y: usize,
    direction: &Direction,
    count: i32,
) -> String {
    (0..=count)
        .map(|step| {
            let x = (x as i32 + step * direction.0) as usize;
            let y = (y as i32 + step * direction.1) as usize;
            matrix.get(x, y)
        })
        .collect()
}

fn part_two(input: &str) -> u32 {
    let matrix = Matrix::from(input);

    let mut total: u32 = 0;

    for (x, y) in matrix.pairs() {
        let forward_slash = check_direction(&matrix, x, y, &(1, 1), 2);
        let back_slash = check_direction(&matrix, x + 2, y, &(-1, 1), 2);

        let mas = String::from("MAS");
        let sam = String::from("SAM");

        if (forward_slash == mas || forward_slash == sam)
            && (back_slash == mas || back_slash == sam)
        {
            total += 1
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
