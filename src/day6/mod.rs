use std::{collections::HashSet, hash::Hash, time::Instant};

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

fn part_one(input: &str) -> u32 {
    let matrix = Matrix::from(input);

    let obstacles = matrix.find_positions("#".to_string());

    let mut guard = matrix
        .find_position_by(|v| v == "^")
        .map(Guard::new)
        .expect("Couldn't find guard position");

    walk_the_guard(&mut guard, &obstacles, &matrix.get_bounding_box());

    guard.visited.len() as u32
}

fn walk_the_guard(
    guard: &mut Guard,
    obstacles: &ObstaclePositions,
    bounding_box: &BoundingBox,
) -> WalkResult {
    let mut result = WalkResult::WALKING;

    while result != WalkResult::OUT_OF_BOUNDS {
        result = guard.walk_toward(&obstacles, bounding_box);

        if result == WalkResult::LOOP {
            break;
        }
    }

    result
}

fn part_two(input: &str) -> u32 {
    let matrix = Matrix::from(input);

    let original_obstacles = matrix.find_positions("#".to_string());

    let mut guard = matrix
        .find_position_by(|v| v == "^")
        .map(Guard::new)
        .expect("Couldn't find guard position");

    walk_the_guard(&mut guard, &original_obstacles, &matrix.get_bounding_box());

    let posititions_to_try = &guard
        .visited
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();

    let mut total = 0;

    for position in posititions_to_try.iter() {
        guard.reset();

        if (position.0 as i32, position.1 as i32) == guard.position
            || original_obstacles.contains(&position)
        {
            continue;
        }

        let obstacles = original_obstacles.with(&position);

        let walk_result = walk_the_guard(&mut guard, &obstacles, &matrix.get_bounding_box());

        if walk_result == WalkResult::LOOP {
            total += 1;
        }
    }

    total
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Guard {
    original_position: (i32, i32),
    position: (i32, i32),
    facing: Direction,
    visited: HashSet<(i32, i32)>,
    walked: HashSet<(i32, i32, i32, i32)>,
}

#[derive(Debug, PartialEq, Eq)]
enum WalkResult {
    LOOP,
    WALKING,
    OUT_OF_BOUNDS,
}

impl Guard {
    fn new((x, y): (usize, usize)) -> Self {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        visited.insert((x as i32, y as i32));

        Self {
            original_position: (x as i32, y as i32),
            position: (x as i32, y as i32),
            facing: Direction::North,
            visited,
            walked: HashSet::new(),
        }
    }

    fn face(&mut self, direction: Direction) {
        self.facing = direction
    }

    fn reset(&mut self) {
        self.visited = HashSet::new();
        self.walked = HashSet::new();
        self.position = self.original_position;
        self.face(Direction::North);
    }

    fn walk_toward(
        &mut self,
        obstacles: &ObstaclePositions,
        bounding_box: &BoundingBox,
    ) -> WalkResult {
        if let Some(pos) = obstacles.find_next_from_point(
            (self.position.0 as usize, self.position.1 as usize),
            &self.facing,
        ) {
            let (obs_x, obs_y) = (pos.0 as i32, pos.1 as i32);
            let path = (self.position.0, self.position.1, obs_x, obs_y);

            if self.walked.contains(&path) {
                return WalkResult::LOOP;
            }

            self.walked.insert(path);

            match self.facing {
                Direction::North => {
                    let distance = obs_y.abs_diff(self.position.1);
                    for _ in 0..distance - 1 {
                        self.position.1 -= 1;
                        self.visited.insert(self.position.clone());
                    }

                    self.facing = Direction::East;
                }
                Direction::East => {
                    let distance = obs_x.abs_diff(self.position.0);
                    for _ in 0..distance - 1 {
                        self.position.0 += 1;
                        self.visited.insert(self.position.clone());
                    }

                    self.facing = Direction::South;
                }
                Direction::South => {
                    let distance = obs_y.abs_diff(self.position.1);
                    for _ in 0..distance - 1 {
                        self.position.1 += 1;
                        self.visited.insert(self.position.clone());
                    }

                    self.facing = Direction::West;
                }
                Direction::West => {
                    let distance = obs_x.abs_diff(self.position.0);
                    for _ in 0..distance - 1 {
                        self.position.0 -= 1;
                        self.visited.insert(self.position.clone());
                    }

                    self.facing = Direction::North;
                }
            }
            return WalkResult::WALKING;
        } else {
            match self.facing {
                Direction::North => {
                    let distance = bounding_box.top_left.1.abs_diff(self.position.1 as usize);
                    for _ in 0..distance {
                        self.position.1 -= 1;
                        self.visited.insert(self.position.clone());
                    }
                }
                Direction::East => {
                    let distance = bounding_box
                        .bottom_right
                        .0
                        .abs_diff(self.position.0 as usize);
                    for _ in 0..distance {
                        self.position.0 += 1;
                        self.visited.insert(self.position.clone());
                    }
                }
                Direction::South => {
                    let distance = bounding_box
                        .bottom_right
                        .1
                        .abs_diff(self.position.1 as usize);
                    for _ in 0..distance {
                        self.position.1 += 1;
                        self.visited.insert(self.position.clone());
                    }
                }
                Direction::West => {
                    let distance = bounding_box.top_left.0.abs_diff(self.position.0 as usize);
                    for _ in 0..distance {
                        self.position.0 -= 1;
                        self.visited.insert(self.position.clone());
                    }
                }
            }
        }
        return WalkResult::OUT_OF_BOUNDS;
    }
}

#[derive(Debug)]
struct ObstaclePositions(HashSet<(usize, usize)>);

impl ObstaclePositions {
    fn new() -> Self {
        ObstaclePositions(HashSet::new())
    }

    fn insert(&mut self, position: (usize, usize)) {
        self.0.insert(position);
    }

    fn with(&self, position: &(usize, usize)) -> Self {
        let mut obstacle_set = self.0.clone();

        obstacle_set.insert(*position);

        Self(obstacle_set)
    }

    fn contains(&self, position: &(usize, usize)) -> bool {
        self.0.contains(position)
    }

    fn find_next_from_point(
        &self,
        position: (usize, usize),
        direction: &Direction,
    ) -> Option<&(usize, usize)> {
        return match direction {
            Direction::North => {
                self.0
                    .iter()
                    .filter(|(pos_x, pos_y)| &position.0 == pos_x && pos_y < &position.1)
                    // min by y
                    .max_by(|a, b| a.1.cmp(&b.1))
            }
            Direction::East => {
                self.0
                    .iter()
                    .filter(|(pos_x, pos_y)| &position.0 < pos_x && pos_y == &position.1)
                    // min by x
                    .min_by(|a, b| a.0.cmp(&b.0))
            }
            Direction::South => {
                self.0
                    .iter()
                    .filter(|(pos_x, pos_y)| &position.0 == pos_x && pos_y > &position.1)
                    // min by y
                    .min_by(|a, b| a.1.cmp(&b.1))
            }
            Direction::West => {
                self.0
                    .iter()
                    .filter(|(pos_x, pos_y)| &position.0 > pos_x && pos_y == &position.1)
                    // max by x
                    .max_by(|a, b| a.0.cmp(&b.0))
            }
            _ => unreachable!(),
        };
    }
}

impl Matrix<String> {
    fn find_positions(&self, to_find: String) -> ObstaclePositions {
        let mut positions = ObstaclePositions::new();

        for (value, position) in self.iter_with_pos() {
            if value == to_find {
                positions.insert(position);
            }
        }

        positions
    }

    fn in_bounds(&self, position: (i32, i32)) -> bool {
        let (x, y) = position;

        return x > 0 || y > 0 || (x as usize) < self.n_columns() || (y as usize) < self.n_rows();
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Guard, ObstaclePositions};

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");
    const EXAMPLE_LOOP_INPUT: &str = include_str!("./example_loop_input.txt");

    #[test]
    fn part_one__example_returns_the_correct_answer() {
        let expected = 41;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 6;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    mod obstacle_positions {
        use crate::day6::Matrix;

        use super::EXAMPLE_INPUT;

        #[test]
        fn obstacle_positions_should_pick_up_all_obstacles_from_example_input() {
            let matrix = Matrix::from(EXAMPLE_INPUT);

            assert_eq!(matrix.n_columns(), 10);
            assert_eq!(matrix.n_rows(), 10);

            let obstacle_positions = matrix.find_positions("#".to_string());

            assert_eq!(8, obstacle_positions.0.len())
        }
    }

    mod guards {

        use ntest::timeout;

        use crate::day6::{tests::EXAMPLE_LOOP_INPUT, BoundingBox, Matrix, WalkResult};

        use super::{Direction, Guard, ObstaclePositions};

        fn create_guard_and_obstacles() -> (Guard, ObstaclePositions) {
            let mut obstacles = ObstaclePositions::new();

            let NORTH = (3, 0);
            let EAST = (6, 3);
            let SOUTH = (3, 6);
            let WEST = (0, 3);

            obstacles.insert(NORTH);
            obstacles.insert(EAST);
            obstacles.insert(SOUTH);
            obstacles.insert(WEST);

            let guard = Guard::new((3, 3));

            (guard, obstacles)
        }
        #[test]
        fn test_guard_walks_north() {
            let (mut guard, obstacles) = create_guard_and_obstacles();

            guard.walk_toward(&obstacles, &BoundingBox::default());

            assert_eq!(guard.visited.len(), 3);
            assert_eq!(guard.facing, Direction::East);
        }

        #[test]
        fn test_guard_walks_east() {
            let (mut guard, obstacles) = create_guard_and_obstacles();
            guard.face(Direction::East);
            guard.walk_toward(&obstacles, &BoundingBox::default());

            assert_eq!(guard.visited.len(), 3);
            assert_eq!(guard.facing, Direction::South);
        }

        #[test]
        fn test_guard_walks_south() {
            let (mut guard, obstacles) = create_guard_and_obstacles();
            guard.face(Direction::South);
            guard.walk_toward(&obstacles, &BoundingBox::default());

            assert_eq!(guard.visited.len(), 3);
            assert_eq!(guard.facing, Direction::West);
        }

        #[test]
        fn test_guard_walks_west() {
            let (mut guard, obstacles) = create_guard_and_obstacles();
            guard.face(Direction::West);
            guard.walk_toward(&obstacles, &BoundingBox::default());

            assert_eq!(guard.visited.len(), 3);
            assert_eq!(guard.facing, Direction::North);
        }

        #[test]
        #[timeout(100)]
        fn test_guard_walks_in_loop() {
            let matrix = Matrix::from(EXAMPLE_LOOP_INPUT);

            let obstacles = matrix.find_positions("#".to_string());

            let mut guard = matrix
                .find_position_by(|v| v == "^")
                .map(Guard::new)
                .expect("Couldn't find guard position");

            let mut inbounds = WalkResult::WALKING;

            while inbounds != WalkResult::OUT_OF_BOUNDS {
                inbounds = guard.walk_toward(&obstacles, &matrix.get_bounding_box());

                if inbounds == WalkResult::LOOP {
                    println!("loop detected");
                    break;
                }
            }
        }
    }
    #[test]
    fn test_find_next_obstacle() {
        let mut positions = ObstaclePositions::new();

        let NORTH = (3, 0);
        let EAST = (6, 3);
        let SOUTH = (3, 6);
        let WEST = (0, 3);

        positions.insert(NORTH);
        positions.insert(EAST);
        positions.insert(SOUTH);
        positions.insert(WEST);

        let position = (3, 3);

        assert_eq!(
            &NORTH,
            positions
                .find_next_from_point(position, &Direction::North)
                .unwrap()
        );
        assert_eq!(
            &EAST,
            positions
                .find_next_from_point(position, &Direction::East)
                .unwrap()
        );
        assert_eq!(
            &SOUTH,
            positions
                .find_next_from_point(position, &Direction::South)
                .unwrap()
        );
        assert_eq!(
            &WEST,
            positions
                .find_next_from_point(position, &Direction::West)
                .unwrap()
        );
    }

    #[test]
    fn test_find_absolute_next_obstacle() {
        let mut positions = ObstaclePositions::new();

        let NORTH = (4, 1);
        let EAST = (7, 4);
        let SOUTH = (4, 7);
        let WEST = (1, 4);

        positions.insert(NORTH);
        positions.insert((4, 0));
        positions.insert(EAST);
        positions.insert((8, 4));
        positions.insert(SOUTH);
        positions.insert((4, 8));
        positions.insert(WEST);
        positions.insert((0, 4));

        let position = (4, 4);

        assert_eq!(
            &NORTH,
            positions
                .find_next_from_point(position, &Direction::North)
                .unwrap()
        );
        assert_eq!(
            &EAST,
            positions
                .find_next_from_point(position, &Direction::East)
                .unwrap()
        );
        assert_eq!(
            &SOUTH,
            positions
                .find_next_from_point(position, &Direction::South)
                .unwrap()
        );
        assert_eq!(
            &WEST,
            positions
                .find_next_from_point(position, &Direction::West)
                .unwrap()
        );
    }
}
