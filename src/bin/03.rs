use std::collections::HashSet;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_to_value(c: char) -> usize {
    ALPHABET
        .chars()
        .position(|alph_char| alph_char == c)
        .unwrap()
        + 1
}

fn compute_score(chars: Vec<char>) -> u32 {
    chars
        .iter()
        .map(|sack_char| char_to_value(*sack_char))
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let matched_chars: Vec<char> = input
        .lines()
        .map(|sack| {
            let left: HashSet<char> = sack[..sack.len() / 2].chars().collect();
            let right: HashSet<char> = sack[sack.len() / 2..].chars().collect();
            // first char in left that matches one in right
            // (clone copies value to avoid a "return reference to local variable" issue)
            *left.intersection(&right).collect::<Vec<&char>>()[0]
        })
        .collect();

    Some(compute_score(matched_chars))
}

pub fn part_two(input: &str) -> Option<u32> {
    // I got into a huge fight with referencing, ownership etc. here, and find it
    // very frustrating that you can't just do x.intersection(y).intersection(z)
    // time to readd more of the rust book...
    let mut matched_chars: Vec<char> = Vec::new();
    for sacks in input.lines().collect::<Vec<&str>>().chunks(3) {
        let mut badge: HashSet<char> = sacks[0].chars().collect();
        for next_sack in &sacks[1..] {
            badge.retain(|x| next_sack.contains(*x));
        }
        matched_chars.push(badge.into_iter().collect::<Vec<char>>()[0]);
    }

    Some(compute_score(matched_chars))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
