use itertools::Itertools;

struct Elf {
    food: Vec<u32>,
}
impl Elf {
    fn total_calories(&self) -> u32 {
        self.food.iter().sum()
    }
}

fn parse_elves(input: &str) -> Vec<Elf> {
    let split: Vec<Vec<&str>> = input
        .split("\n\n") // each elf separated by blank line
        .map(|s| s.trim().split('\n').collect()) // one food item on each line
        .collect();

    split
        .iter()
        .map(|s| Elf {
            // construct elf from vec of food
            food: s.iter().map(|x| x.parse::<u32>().unwrap()).collect(), // vec of str to vec of i32
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = parse_elves(input);
    elves.iter().map(|e| e.total_calories()).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = parse_elves(input);
    let mut calories: Vec<u32> = elves.iter().map(|e| e.total_calories()).collect_vec();
    calories.sort();
    Some(calories[calories.len() - 3..].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
