use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

struct Instruction {
    n: usize,
    origin: usize,
    destination: usize,
}
fn parse_instruction(instr: &str) -> Instruction {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let matches = &re.captures_iter(instr).collect_vec()[0];
    Instruction {
        n: matches[1].parse::<usize>().unwrap(),
        origin: matches[2].parse::<usize>().unwrap(),
        destination: matches[3].parse::<usize>().unwrap(),
    }
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Instruction>) {
    // separate input into stacks and instructions
    let sections = input.split("\n\n").collect_vec();

    // parse stacks
    let mut stacks = Vec::<VecDeque<char>>::new();
    for lin in sections[0].lines() {
        let mut column: usize = 0;
        for (cursor, chr) in lin.chars().enumerate() {
            if chr.is_numeric() {
                // last line (column labels)
                break;
            }
            if cursor % 4 == 0 {
                // new column every 4 characters (format is: [A]_ )
                // check we have enough vectors and initiate a new one if not
                if stacks.len() <= column {
                    stacks.push(VecDeque::<char>::new())
                }
                column += 1;
            }
            if chr.is_alphabetic() {
                // add this crate's character to the stack
                stacks[column - 1].push_back(chr);
            }
        }
    }
    // parse instructions
    let instructions = sections[1].lines().map(parse_instruction).collect_vec();

    (stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, instructions) = parse_input(input);
    for instr in instructions {
        let mut moved = VecDeque::<char>::new();
        // remove number of requested characters from the origin stack
        for _ in 0..instr.n {
            moved.push_front(stacks[instr.origin - 1].pop_front().unwrap());
        }
        // then append them to the destination stack
        moved.append(&mut stacks[instr.destination - 1]);
        stacks[instr.destination - 1] = moved;
    }

    Some(stacks.iter().map(|s| s[0]).collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    // the only difference between this and part_one is moved.push_back instead
    // of moved.push_front in the for loop below (will tidy up later)
    let (mut stacks, instructions) = parse_input(input);
    for instr in instructions {
        let mut moved = VecDeque::<char>::new();
        // remove number of requested characters from the origin stack
        for _ in 0..instr.n {
            moved.push_back(stacks[instr.origin - 1].pop_front().unwrap());
        }
        // then append them to the destination stack
        moved.append(&mut stacks[instr.destination - 1]);
        stacks[instr.destination - 1] = moved;
    }

    Some(stacks.iter().map(|s| s[0]).collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
