use itertools::Itertools;

struct Monkey {
    items: Vec<u64>,
    operation: String,
    test: String,
    inspect_count: u64,
}
impl Monkey {
    fn inspect(&mut self, part1: bool, max_worry: u64) {
        // run operation
        let terms = self
            .operation
            .trim()
            .strip_prefix("Operation: new = ")
            .unwrap()
            .split(' ')
            .collect_vec();
        for worry in &mut self.items {
            // perform inspections
            let lhs = match terms[0] {
                "old" => *worry,
                _ => terms[0].parse::<u64>().unwrap(),
            };
            let rhs = match terms[2] {
                "old" => *worry,
                _ => terms[2].parse::<u64>().unwrap(),
            };
            *worry = match terms[1] {
                "*" => lhs * rhs,
                "+" => lhs + rhs,
                _ => panic!("unknown operation"),
            };
            if part1 {
                *worry /= 3; // divide worry by 3 after inspection
            }
            *worry %= max_worry; // avoid overflow using modulo arithmetic
            self.inspect_count += 1;
        }
    }
    fn throw(&mut self) -> Vec<(usize, u64)> {
        let test_lines = self
            .test
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .lines()
            .collect_vec();

        let div_by = test_lines[0].parse::<u64>().unwrap();

        let true_idx = test_lines[1]
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_idx = test_lines[2]
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let thrown_items = self
            .items
            .iter()
            .map(|worry| {
                if *worry % div_by == 0 {
                    (true_idx, *worry)
                } else {
                    (false_idx, *worry)
                }
            })
            .collect_vec();
        self.items = Vec::<u64>::new(); // empty items as we throw them all to others

        thrown_items
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let groups = input.split("\n\n").collect_vec();
    let mut monkeys = Vec::<Monkey>::new();
    for g in groups {
        let lines = g.lines().collect_vec();
        let items = lines[1]
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect_vec();
        let operation = lines[2].trim().to_string();
        let test = lines[3..].join("\n").trim().to_string();
        monkeys.push(Monkey {
            items,
            operation,
            test,
            inspect_count: 0,
        });
    }
    monkeys
}

fn run(input: &str, part1: bool) -> u64 {
    let mut monkeys = parse_monkeys(input);

    // will use worry values module common factor of all divisors to avoid overflow
    let max_worry: u64 = monkeys
        .iter()
        .map(|m| {
            // parse divide by value from test string
            m.test
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .lines()
                .collect_vec()[0]
                .parse::<u64>()
                .unwrap()
        })
        .product();

    let rounds = if part1 { 20 } else { 10000 };
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            monkeys[monkey_idx].inspect(part1, max_worry);
            let thrown_items = monkeys[monkey_idx].throw();
            for item in thrown_items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }
    let counts = monkeys
        .iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .collect::<Vec<u64>>();

    counts[0] * counts[1]
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(run(input, true))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run(input, false))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
