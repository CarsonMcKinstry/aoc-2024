use regex::{Captures, Match, Regex};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY THREE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

fn part_one(input: &str) -> u32 {
    let mul_captures_re = Regex::new(r"mul\((?<lhs>\d{1,3})\,(?<rhs>\d{1,3})\)")
        .expect("failed to create regex for part 1");

    mul_captures_re
        .captures_iter(input)
        .map(|cap| {
            let lhs: u32 = cap["lhs"].parse().expect("failed to parse lhs");
            let rhs: u32 = cap["rhs"].parse().expect("f?ailed to parse rhs");

            lhs * rhs
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut include = true;
    let mul_re =
        Regex::new(r"mul\((?<lhs>\d{1,3})\,(?<rhs>\d{1,3})\)").expect("failed to create mul regex");
    let inst_re =
        Regex::new(r"(?<inst>do(n\'t)?\(\))").expect("failed to create instruction regex");

    let mul_captures = mul_re.captures_iter(input);
    let inst_captures = inst_re.captures_iter(input);

    let mut captures = mul_captures.chain(inst_captures).collect::<Vec<Captures>>();

    captures.sort_by(|cap_a, cap_b| {
        let cap_a_start = cap_a.get(0).unwrap().start();
        let cap_b_start = cap_b.get(0).unwrap().start();

        cap_a_start.cmp(&cap_b_start)
    });

    captures
        .into_iter()
        .map(|cap| {
            let instruction = cap.name("inst");

            include = match instruction {
                Some(m) if m.as_str() == "don't()" => false,
                Some(m) if m.as_str() == "do()" => true,
                _ => include,
            };

            if !include {
                return 0;
            }

            let lhs: u32 = cap
                .name("lhs")
                .map_or(0, |n| n.as_str().parse().expect("failed to parse lhs"));
            let rhs: u32 = cap
                .name("rhs")
                .map_or(0, |n| n.as_str().parse().expect("failed to parse rhs"));

            lhs * rhs
        })
        .sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;

        let actual = super::part_one(input);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 48;

        let actual = super::part_two(input);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_complicated_example_returns_the_correct_answer() {
        let input = "xmul(8,5)&mul[3,7]!^don't()don't()do()don't()
_mul(500,15)don't()+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 80;

        let actual = super::part_two(input);

        assert_eq!(actual, expected)
    }
}
