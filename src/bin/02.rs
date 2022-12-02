use std::collections::HashMap;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

pub fn part_one(input: &str) -> Option<u32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("A X"), ROCK + DRAW); // them Rock, me Rock DRAW
    scores.insert(String::from("A Y"), PAPER + WIN); // them Rock, me Paper WIN
    scores.insert(String::from("A Z"), SCISSORS + LOSS); // them Rock, me Scissors LOSS
    scores.insert(String::from("B X"), ROCK + LOSS); // them Paper, me Rock LOSS
    scores.insert(String::from("B Y"), PAPER + DRAW); // them Paper, me Paper DRAW
    scores.insert(String::from("B Z"), SCISSORS + WIN); // them Paper, me Scissors WIN
    scores.insert(String::from("C X"), ROCK + WIN); // them Scissors, me Rock WIN
    scores.insert(String::from("C Y"), PAPER + LOSS); // them Scissors, me Paper LOSS
    scores.insert(String::from("C Z"), SCISSORS + DRAW); // them Scissors, me Scissors DRAW

    Some(
        input
            .trim()
            .split('\n')
            .map(|s| scores.get(s).unwrap())
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("A X"), SCISSORS + LOSS); // LOSE against Rock, play Scissors
    scores.insert(String::from("A Y"), ROCK + DRAW); // DRAW against Rock, play Rock
    scores.insert(String::from("A Z"), PAPER + WIN); // WIN against Rock, play Paper
    scores.insert(String::from("B X"), ROCK + LOSS); // LOSE against Paper, play Rock
    scores.insert(String::from("B Y"), PAPER + DRAW); // DRAW against Paper, play Paper
    scores.insert(String::from("B Z"), SCISSORS + WIN); // WIN against Paper, play Scissors
    scores.insert(String::from("C X"), PAPER + LOSS); // LOSE against Scissors, play Paper
    scores.insert(String::from("C Y"), SCISSORS + DRAW); // DRAW against Scissors, play Scissors
    scores.insert(String::from("C Z"), ROCK + WIN); // WIN against Scissors, play Rock

    Some(
        input
            .trim()
            .split('\n')
            .map(|s| scores.get(s).unwrap())
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
