use std::{cmp::Ordering, collections::HashSet, hash::Hash};

use rayon::prelude::*;

use crate::models::position::{Distance, Position};

use std::time::Instant;

use crate::models::matrix::{BoundingBox, Matrix};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY SIX =====");
    let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT);
    println!("({:.2?}) Part 1: {:?}", now.elapsed(), part_one_result);
    let now = Instant::now();
    let part_two_result = part_two(PUZZLE_INPUT);
    println!("({:.2?}) Part 2: {:?}", now.elapsed(), part_two_result);
}

fn part_one(input: &str) -> usize {
    let matrix = Matrix::from(input);
    let obstacles = get_obstacles(&matrix);

    let guard = get_guard_position(&matrix);

    walk_the_guard(&guard, &obstacles, &matrix.get_bounding_box())
        .map_or(0, |visited| visited.len())
}

fn part_two(input: &str) -> usize {
    let matrix = Matrix::from(input);
    let obstacles = get_obstacles(&matrix);

    let guard = get_guard_position(&matrix);

    let potential_obstacles =
        walk_the_guard(&guard, &obstacles, &matrix.get_bounding_box()).unwrap();

    potential_obstacles
        .par_iter()
        .filter_map(|pos| {
            if pos == &guard {
                return None;
            }

            let mut next_obstacles = obstacles.clone();

            next_obstacles.insert(pos.clone());

            match walk_the_guard(&guard, &next_obstacles, &matrix.get_bounding_box()) {
                Some(_) => None,
                None => Some(1),
            }
        })
        .sum()
}

fn walk_the_guard(
    initial_position: &Position,
    obstacles: &HashSet<Position>,
    bounding_box: &BoundingBox,
) -> Option<HashSet<Position>> {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut visited_states: HashSet<(Position, Direction)> = HashSet::new();

    visited.insert(initial_position.clone());

    let mut walk_result = WalkResult {
        finished: false,
        next_direction: Direction::North,
        next_position: initial_position.clone(),
    };

    while !walk_result.finished {
        let current_state = (
            walk_result.next_position.clone(),
            walk_result.next_direction.clone(),
        );

        if visited_states.contains(&current_state) {
            return None;
        }

        visited_states.insert(current_state);

        walk_result = walk(
            &walk_result.next_position,
            obstacles,
            &walk_result.next_direction,
            bounding_box,
        );

        visited.insert(walk_result.next_position);
    }

    Some(visited)
}

fn get_obstacles(matrix: &Matrix<String>) -> HashSet<Position> {
    matrix
        .iter_with_pos()
        .map(|(value, position)| {
            if value == "#" {
                return Some(position);
            } else {
                return None;
            }
        })
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .collect()
}
fn get_guard_position(matrix: &Matrix<String>) -> Position {
    matrix
        .find_position_by(is_guard)
        .expect("Failed to find guard position")
}

fn is_guard(character: String) -> bool {
    character == "^"
}

fn find_next_obstacle(
    obstacles: &HashSet<Position>,
    from_point: &Position,
    direction: &Direction,
) -> Option<Position> {
    let it = obstacles.iter().filter(|pos| match direction {
        Direction::North => pos.north_of(from_point),
        Direction::South => pos.south_of(from_point),
        Direction::East => pos.east_of(from_point),
        Direction::West => pos.west_of(from_point),
    });

    match direction {
        Direction::North => it.max_by(|a, b| a.compare_y(b)),
        Direction::South => it.min_by(|a, b| a.compare_y(b)),
        Direction::East => it.min_by(|a, b| a.compare_x(b)),
        Direction::West => it.max_by(|a, b| a.compare_x(b)),
    }
    .cloned()
}

fn walk(
    current_position: &Position,
    obstacles: &HashSet<Position>,
    direction: &Direction,
    bounding_box: &BoundingBox,
) -> WalkResult {
    let mut visited: HashSet<Position> = HashSet::new();

    let next_position = match direction {
        Direction::North => current_position.add(0, -1),
        Direction::East => current_position.add(1, 0),
        Direction::South => current_position.add(0, 1),
        Direction::West => current_position.add(-1, 0),
    };

    if !bounding_box.contains(&next_position) {
        return WalkResult {
            next_position: current_position.clone(),
            finished: true,
            next_direction: direction.clone(),
        };
    }

    if obstacles.contains(&next_position) {
        return WalkResult {
            next_position: current_position.clone(),
            finished: false,
            next_direction: direction.turn_right(),
        };
    }

    WalkResult {
        next_position,
        finished: false,
        next_direction: direction.clone(),
    }
}

fn walk_forward(position: &Position, direction: &Direction) -> Position {
    match direction {
        Direction::North => position.add(0, -1),
        Direction::East => position.add(1, 0),
        Direction::South => position.add(0, 1),
        Direction::West => position.add(-1, 0),
    }
    .clone()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug)]
struct WalkResult {
    next_position: Position,
    finished: bool,
    next_direction: Direction,
    // visited: HashSet<Position>,
}

#[cfg(test)]
mod tests {

    mod parts {
        const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

        use crate::day6::PUZZLE_INPUT;

        use super::super::{part_one, part_two};

        #[test]
        fn part_one_example_returns_the_correct_answer() {
            let expected = 41;

            let actual = part_one(EXAMPLE_INPUT);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_one_actual_input() {
            let expected = 4776;

            let actual = part_one(PUZZLE_INPUT);

            assert_eq!(actual, expected);
        }

        #[test]
        fn part_two_example_returns_the_correct_answer() {
            let expected = 6;

            let actual = part_two(EXAMPLE_INPUT);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_two_actual_input() {
            let expected = 1586;

            let actual = part_two(PUZZLE_INPUT);

            assert_eq!(actual, expected);
        }
    }

    mod infinite_loops {
        use std::collections::HashSet;

        use ntest::timeout;

        use crate::day6::{walk_the_guard, BoundingBox};

        use super::super::Position;

        #[test]
        #[timeout(10)]
        fn test_should_return_none_for_infinite_loop() {
            let mut obstacles: HashSet<Position> = HashSet::new();

            obstacles.insert(Position::new(1, 0));
            obstacles.insert(Position::new(4, 1));
            obstacles.insert(Position::new(3, 4));
            obstacles.insert(Position::new(0, 3));

            let start_position = Position::new(1, 3);

            let result = walk_the_guard(
                &start_position,
                &obstacles,
                &BoundingBox {
                    top_left: Position::new(0, 0),
                    bottom_right: Position::new(4, 4),
                },
            );

            assert_eq!(result, None);
        }
    }

    mod obstacles {
        use std::collections::HashSet;

        use super::super::{find_next_obstacle, Direction};

        use super::super::Position;

        fn create_obstacles() -> HashSet<Position> {
            let mut obstacles: HashSet<Position> = HashSet::new();
            obstacles.insert(Position::new(2, 0)); // north
            obstacles.insert(Position::new(4, 2)); // east
            obstacles.insert(Position::new(2, 4)); // south
            obstacles.insert(Position::new(0, 2)); // west

            obstacles
        }

        #[test]
        fn test_find_next_obstacle() {
            let obstacles = create_obstacles();

            let from_point = Position::new(2, 2);

            assert_eq!(
                Some(Position::new(2, 0)),
                find_next_obstacle(&obstacles, &from_point, &Direction::North)
            );

            assert_eq!(
                Some(Position::new(2, 4)),
                find_next_obstacle(&obstacles, &from_point, &Direction::South)
            );

            assert_eq!(
                Some(Position::new(4, 2)),
                find_next_obstacle(&obstacles, &from_point, &Direction::East)
            );

            assert_eq!(
                Some(Position::new(0, 2)),
                find_next_obstacle(&obstacles, &from_point, &Direction::West)
            );
        }
    }
}
