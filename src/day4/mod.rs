const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

type Direction = (i32, i32);

struct Matrix(Vec<Vec<String>>);

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
    println!("===== DAY THREE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

impl Matrix {
    fn n_columns(&self) -> usize {
        self.0.first().map(|f| f.len()).unwrap()
    }

    fn n_rows(&self) -> usize {
        self.0.len()
    }

    fn get(&self, x: usize, y: usize) -> String {
        self.0
            .get(y)
            .and_then(|line| line.get(x))
            .cloned()
            .unwrap_or_default()
    }

    fn pairs(&self) -> MatrixCoordinates {
        MatrixCoordinates {
            matrix: self,
            row: 0,
            col: 0,
        }
    }
}

impl From<Vec<Vec<String>>> for Matrix {
    fn from(value: Vec<Vec<String>>) -> Self {
        Self(value)
    }
}

impl From<&str> for Matrix {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
        )
    }
}

struct MatrixCoordinates<'a> {
    matrix: &'a Matrix,
    row: usize,
    col: usize,
}

impl<'a> Iterator for MatrixCoordinates<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.matrix.n_rows() {
            return None;
        }

        let result = (self.row, self.col);

        self.col += 1;

        if self.col >= self.matrix.n_columns() {
            self.col = 0;
            self.row += 1;
        }

        Some(result)
    }
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
    matrix: &Matrix,
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
