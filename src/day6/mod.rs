const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY ONE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    0
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one__example_returns_the_correct_answer() {
        let expected = 11;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 31;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
