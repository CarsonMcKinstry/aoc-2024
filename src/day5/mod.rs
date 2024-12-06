use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const PUZZLE_INPUT: &str = include_str!("./puzzle_input.txt");

pub(crate) fn run() {
    println!("===== DAY THREE =====");
    println!("Part 1: {:?}", part_one(PUZZLE_INPUT));
    println!("Part 2: {:?}", part_two(PUZZLE_INPUT));
}

type PrintOrders = Vec<Vec<u32>>;

struct OrderingRules(HashMap<u32, HashSet<u32>>);

impl OrderingRules {
    fn get(&self, page_number: u32) -> Option<&HashSet<u32>> {
        self.0.get(&page_number)
    }

    fn is_valid_order(&self, order: &Vec<u32>) -> bool {
        for i in 0..order.len() {
            let page = order.get(i).unwrap();

            if let Some(rules) = self.get(*page) {
                for j in 0..order.len() {
                    let next_page = order.get(j).unwrap();

                    if rules.contains(next_page) && j < i {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn repair_order(&self, order: &Vec<u32>) -> Vec<u32> {
        let mut repaired_order = order.clone();

        repaired_order.sort_by(|a, b| {
            if let Some(rules) = self.get(*a) {
                if rules.contains(b) {
                    return Ordering::Greater;
                }
            }
            Ordering::Less
        });

        repaired_order
    }
}

impl From<&&str> for OrderingRules {
    fn from(value: &&str) -> Self {
        let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

        for line in value.lines() {
            let temp = line
                .split("|")
                .take(2)
                .map(|n| n.parse().expect("Failed to parse n in ordering rule"))
                .collect::<Vec<u32>>();

            let key = *temp.get(0).expect("failed to get key");
            let value = *temp.get(1).expect("failed to get value");

            map.entry(key).or_insert_with(HashSet::new).insert(value);
        }

        Self(map)
    }
}

fn part_one(input: &str) -> u32 {
    let (ordering_rules, print_orders) = parse_input(input);

    print_orders
        .iter()
        .filter(|order| ordering_rules.is_valid_order(order))
        .map(take_middle)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let (ordering_rules, print_orders) = parse_input(input);

    print_orders
        .iter()
        .filter(|order| !ordering_rules.is_valid_order(order))
        .map(|order| ordering_rules.repair_order(order))
        .map(|order| take_middle(&order))
        .sum()
}

fn parse_input(input: &str) -> (OrderingRules, PrintOrders) {
    let split = input.split("\n\n").take(2).collect::<Vec<&str>>();

    let ordering_rules = split
        .get(0)
        .map(OrderingRules::from)
        .expect("Failed to get ordering rules");

    let print_orders = split
        .get(1)
        .map(|orders| {
            orders
                .lines()
                .map(|line| {
                    line.split(",")
                        .map(|page_number| {
                            page_number.parse().expect("failed to parse page number")
                        })
                        .collect()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .expect("Failed to get print orders");

    (ordering_rules, print_orders)
}

fn take_middle(order: &Vec<u32>) -> u32 {
    let middle_index = order.len() / 2;

    *order
        .get(middle_index)
        .expect("unable to find the middle value")
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = include_str!("./example_input.txt");

    #[test]
    fn part_one_example_returns_the_correct_answer() {
        let expected = 143;

        let actual = super::part_one(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn part_two_example_returns_the_correct_answer() {
        let expected = 123;

        let actual = super::part_two(EXAMPLE_INPUT);

        assert_eq!(actual, expected)
    }
}
