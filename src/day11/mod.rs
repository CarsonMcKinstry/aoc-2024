use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY ELEVEN =====");
    // let now = Instant::now();
    let part_one_result = part_one(PUZZLE_INPUT, 25);
    println!("Part 1: {:?}", part_one_result);
    let part_two_result = part_one(PUZZLE_INPUT, 75);
    println!("Part 2: {:?}", part_two_result);
}

#[time_it::time_it]
fn part_one(input: &str, n_blinks: usize) -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in input.split_whitespace() {
        let num: u64 = stone.parse().expect("Input contains a non-integer value");
        *stones.entry(num).or_insert(0) += 1;
    }

    blink(stones, n_blinks, 0).values().sum()
}

fn blink(stones: HashMap<u64, u64>, n_blinks: usize, curr_blinks: usize) -> HashMap<u64, u64> {
    if curr_blinks == n_blinks {
        return stones;
    }

    let mut new_stones = HashMap::with_capacity(stones.len());
    for (k, v) in stones {
        match k {
            0 => *new_stones.entry(1).or_insert(0) += v,
            1 => *new_stones.entry(2024).or_insert(0) += v,
            k => {
                let num = format!("{}", k);
                if num.len() % 2 > 0 {
                    *new_stones.entry(2024 * k).or_insert(0) += v;
                } else {
                    let l: u64 = num[..num.len() / 2].parse().unwrap();
                    let r: u64 = num[num.len() / 2..].parse().unwrap();
                    *new_stones.entry(l).or_insert(0) += v;
                    *new_stones.entry(r).or_insert(0) += v;
                }
            }
        };
    }

    blink(new_stones, n_blinks, curr_blinks + 1)
}

fn count_digits(n: &u64) -> u32 {
    let mut n_digits: u32 = 0;
    let mut temp = *n;

    while temp > 0 {
        n_digits += 1;
        temp /= 10;
    }

    n_digits
}

#[cfg(test)]
mod tests {
    use super::part_one;
    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    mod part_one {
        use super::EXAMPLE_INPUT;

        #[test]
        fn part_one_example_returns_the_correct_answer() {
            let expected = 55312;

            let actual = super::part_one(EXAMPLE_INPUT, 25);

            assert_eq!(actual, expected)
        }

        #[test]
        fn part_one_simple() {
            let expected = 22;

            let actual = super::part_one(EXAMPLE_INPUT, 6);

            assert_eq!(actual, expected);
        }
    }
}
