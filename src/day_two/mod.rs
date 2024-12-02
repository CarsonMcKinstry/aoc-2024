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
                .map(|n| n.parse::<i32>().expect("failed to parse value in report"))
                .collect::<Vec<i32>>()
        })
        .filter(report_is_safe)
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("failed to parse value in report"))
                .collect::<Vec<i32>>()
        })
        .filter(report_is_safe_with_dampening)
        .count()
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff >= 1 && diff <= 3
    });
    let is_decreasing = report.windows(2).all(|w| {
        let diff = w[0] - w[1];
        diff >= 1 && diff <= 3
    });

    is_decreasing || is_increasing
}

fn report_is_safe_with_dampening(report: &Vec<i32>) -> bool {
    if report_is_safe(&report) {
        return true;
    }

    for i in 0..report.len() {
        let mut dampened_levels = report.clone();
        dampened_levels.remove(i);

        if report_is_safe(&dampened_levels) {
            return true;
        }
    }

    false
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
