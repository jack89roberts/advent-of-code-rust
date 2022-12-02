use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {

    let mut scores = HashMap::new();
    scores.insert(String::from("A X"), 1 + 3); // them Rock, me Rock DRAW
    scores.insert(String::from("A Y"), 2 + 6); // them Rock, me Paper WIN
    scores.insert(String::from("A Z"), 3 + 0); // them Rock, me Scissors LOSS
    scores.insert(String::from("B X"), 1 + 0); // them Paper, me Rock LOSS
    scores.insert(String::from("B Y"), 2 + 3); // them Paper, me Paper DRAW
    scores.insert(String::from("B Z"), 3 + 6); // them Paper, me Scissors WIN
    scores.insert(String::from("C X"), 1 + 6); // them Scissors, me Rock WIN
    scores.insert(String::from("C Y"), 2 + 0); // them Scissors, me Paper LOSS
    scores.insert(String::from("C Z"), 3 + 3); // them Scissors, me Scissors DRAW
    
    Some(input.trim().split('\n').map(|s| scores.get(s).unwrap()).sum::<u32>())

}

pub fn part_two(input: &str) -> Option<u32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("A X"), 3 + 0); // LOSE against Rock, play Scissors
    scores.insert(String::from("A Y"), 1 + 3); // DRAW against Rock, play Rock
    scores.insert(String::from("A Z"), 2 + 6); // WIN against Rock, play Paper
    scores.insert(String::from("B X"), 1 + 0); // LOSE against Paper, play Rock
    scores.insert(String::from("B Y"), 2 + 3); // DRAW against Paper, play Paper
    scores.insert(String::from("B Z"), 3 + 6); // WIN against Paper, play Scissors
    scores.insert(String::from("C X"), 2 + 0); // LOSE against Scissors, play Paper
    scores.insert(String::from("C Y"), 3 + 3); // DRAW against Scissors, play Scissors
    scores.insert(String::from("C Z"), 1 + 6); // WIN against Scissors, play Rock
    
    Some(input.trim().split('\n').map(|s| scores.get(s).unwrap()).sum::<u32>())
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
