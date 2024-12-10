use std::{collections::HashSet, time::Instant};

use crate::models::point::Point;

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY TEN =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_two(PUZZLE_INPUT);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

const NORTH: &Point = &Point(0, -1);
const SOUTH: &Point = &Point(0, 1);
const EAST: &Point = &Point(1, 0);
const WEST: &Point = &Point(-1, 0);

fn part_one(input: &str) -> usize {
    let map = TopographicMap::from(input);

    map.find_all(0)
        .iter()
        .map(|point| {
            let mut peaks = HashSet::<Point>::new();
            peaks = find_num_paths(0, point, &map, &mut peaks);

            peaks.len()
        })
        .sum::<usize>()
}

fn find_num_paths(
    current_value: u8,
    starting_point: &Point,
    map: &TopographicMap,
    peaks: &mut HashSet<Point>,
) -> HashSet<Point> {
    if current_value == 9 {
        peaks.insert(*starting_point);

        return peaks.clone();
    }

    let points_to_check = vec![
        *starting_point + *NORTH,
        *starting_point + *EAST,
        *starting_point + *SOUTH,
        *starting_point + *WEST,
    ];

    for point in points_to_check {
        if let Some(value) = map.get(point) {
            if *value == current_value + 1 {
                let result = find_num_paths(*value, &point, map, peaks);

                peaks.extend(result);
            }
        }
    }

    return peaks.clone();
}

fn part_two(input: &str) -> usize {
    let map = TopographicMap::from(input);

    map.find_all(0)
        .iter()
        .map(|point| {
            let mut path = Vec::<Point>::new();
            let mut unique_paths = HashSet::<Vec<Point>>::new();
            unique_paths = find_unique_paths(0, point, &map, &mut path, &mut unique_paths);

            unique_paths.len()
        })
        .sum::<usize>()
}

fn find_unique_paths(
    current_value: u8,
    starting_point: &Point,
    map: &TopographicMap,
    path: &mut Vec<Point>,
    unique_paths: &mut HashSet<Vec<Point>>,
) -> HashSet<Vec<Point>> {
    if current_value == 9 {
        unique_paths.insert(path.clone());

        return unique_paths.clone();
    }

    let points_to_check = vec![
        *starting_point + *NORTH,
        *starting_point + *EAST,
        *starting_point + *SOUTH,
        *starting_point + *WEST,
    ];

    for point in points_to_check {
        if let Some(value) = map.get(point) {
            if *value == current_value + 1 && !path.contains(&point) {
                path.push(point);

                find_unique_paths(*value, &point, map, path, unique_paths);

                path.pop();
            }
        }
    }

    return unique_paths.clone();
}

struct TopographicMap {
    pub height: i32,
    pub width: i32,

    map: Vec<u8>,
}

impl From<&str> for TopographicMap {
    fn from(value: &str) -> Self {
        let width = value.find('\n').unwrap() as i32;
        let height = value.chars().filter(|c| *c == '\n').count() as i32 + 1;

        let map = value
            .replace('\n', "")
            .chars()
            .map(|c| c.to_string().parse::<u8>().expect("failed to parse height"))
            .collect::<Vec<u8>>();

        Self { width, height, map }
    }
}

impl TopographicMap {
    fn get(&self, point: Point) -> Option<&u8> {
        if point.x() < 0 || point.x() >= self.width || point.y() < 0 || point.y() >= self.height {
            None
        } else {
            let i = point.y() * self.width + point.x();

            self.map.get(i as usize)
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn find_all(&self, value: u8) -> Vec<Point> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if *v == value {
                    let col = i as i32 % self.width;
                    let row = i as i32 / self.width;

                    let point = Point::from((col, row));
                    Some(point)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};
    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    mod part_one {
        use super::EXAMPLE_INPUT;

        #[test]
        fn part_one_example_returns_the_correct_answer() {
            let expected = 36;

            let actual = super::part_one(EXAMPLE_INPUT);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_one_simple() {
            let expected = 1;

            let actual = super::part_one(
                r"0123
1234
8765
9876",
            );

            assert_eq!(actual, expected);
        }

        #[test]
        fn part_one_branching() {
            let expected = 2;

            let actual = super::part_one(
                r"2220222
3331333
4442444
6543456
7999997
8777778
9777779",
            );

            assert_eq!(actual, expected);
        }
    }

    mod part_two {

        use super::EXAMPLE_INPUT;

        #[test]
        fn part_two_example_returns_the_correct_answer() {
            let expected = 81;

            let actual = super::part_two(EXAMPLE_INPUT);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_two_branching() {
            let expected = 2;

            let actual = super::part_one(
                r"2220222
3331333
4442444
6543456
7999997
8777778
9777779",
            );

            assert_eq!(actual, expected);
        }
    }
}
