use itertools::Itertools;

struct Elf {
    low: u32,
    high: u32,
}
impl Elf {
    fn contains(&self, other: &Elf) -> bool {
        self.low <= other.low && self.high >= other.high
    }
}
impl Elf {
    fn overlaps(&self, other: &Elf) -> bool {
        other.low >= self.low && other.low <= self.high
            || self.low >= other.low && self.low <= other.high
    }
}

fn parse_elves(input: &str) -> Vec<Vec<Elf>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| {
                    let elf_values = elf
                        .split('-')
                        .map(|digit| digit.parse::<u32>().unwrap())
                        .collect_vec();
                    Elf {
                        low: elf_values[0],
                        high: elf_values[1],
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_pairs = parse_elves(input);
    let contained_pairs = elf_pairs.iter().fold(0, |total, pair| {
        if pair[0].contains(&pair[1]) || pair[1].contains(&pair[0]) {
            total + 1
        } else {
            total
        }
    });
    Some(contained_pairs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_pairs = parse_elves(input);
    let overlapping_pairs = elf_pairs.iter().fold(0, |total, pair| {
        if pair[0].overlaps(&pair[1]) {
            total + 1
        } else {
            total
        }
    });
    Some(overlapping_pairs)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
