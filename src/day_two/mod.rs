const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY TWO =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().expect("failed to parse value in report"))
                .collect::<Vec<u32>>()
        })
        .filter(report_is_safe_part_one)
        .count()
}

#[derive(PartialEq, Eq)]
enum Direction {
    Inc,
    Dec,
}

fn report_is_safe_part_one(report: &Vec<u32>) -> bool {
    let mut direction: Option<Direction> = None;

    for i in 0..report.len() - 1 {
        let curr = report.get(i).unwrap();
        let next = report.get(i + 1).unwrap();

        let dif = curr.abs_diff(*next);

        if dif < 1 || dif > 3 {
            return false;
        }

        let next_direction = if curr < next {
            Direction::Inc
        } else {
            Direction::Dec
        };

        match direction {
            Some(Direction::Dec) => {
                if next_direction != Direction::Dec {
                    return false;
                }
            }
            Some(Direction::Inc) => {
                if next_direction != Direction::Inc {
                    return false;
                }
            }
            None => direction = Some(next_direction),
        }
    }
    true
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u32>().expect("failed to parse value in report"))
                .collect::<Vec<u32>>()
        })
        .filter(report_is_safe_part_two)
        .count()
}

fn report_is_safe_part_two(report: &Vec<u32>) -> bool {
    let mut bad_level_found = false;
    let mut direction: Option<Direction> = None;

    for i in 0..report.len() - 1 {
        let curr = report.get(i).unwrap();
        let next = report.get(i + 1).unwrap();

        let dif = curr.abs_diff(*next);

        if dif < 1 || dif > 3 {
            if bad_level_found {
                return false;
            } else {
                bad_level_found = true;
                continue;
            }
        }

        let next_direction = if curr < next {
            Direction::Inc
        } else {
            Direction::Dec
        };

        match direction {
            Some(Direction::Dec) => {
                if next_direction != Direction::Dec {
                    if bad_level_found {
                        return false;
                    } else {
                        bad_level_found = true;
                        continue;
                    }
                }
            }
            Some(Direction::Inc) => {
                if next_direction != Direction::Inc {
                    if bad_level_found {
                        return false;
                    } else {
                        bad_level_found = true;
                        continue;
                    }
                }
            }
            None => direction = Some(next_direction),
        }
    }
    true
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_returns_the_correct_answer() {
        let expected = 2;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_returns_the_correct_answer() {
        let expected = 4;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
