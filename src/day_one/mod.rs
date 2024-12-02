use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY ONE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = split_lists(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part_two(input: &str) -> u32 {
    let (left, right) = split_lists(input);

    let mut right_counts: HashMap<u32, u32> = HashMap::new();

    for value in right {
        if !right_counts.contains_key(&value) {
            right_counts.insert(value, 0);
        }

        right_counts.insert(value, right_counts.get(&value).unwrap() + 1);
    }

    left.iter()
        .map(|value| value * right_counts.get(value).unwrap_or(&0))
        .sum()
}

fn split_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let line = line.split_whitespace();

            let left = line
                .clone()
                .take(1)
                .collect::<Vec<&str>>()
                .first()
                .map(|l| l.parse::<u32>().expect("failed to parse left"))
                .expect("failed to retrieve left hand side");
            let right = line
                .clone()
                .skip(1)
                .take(1)
                .collect::<Vec<&str>>()
                .first()
                .map(|r| r.parse::<u32>().expect("failed to parse right"))
                .expect("failed to retrieve right hand side");

            (left, right)
        })
        .unzip()
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_returns_the_correct_answer() {
        let expected = 11;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_returns_the_correct_answer() {
        let expected = 31;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
