use itertools::Itertools;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Debug, Clone)]
enum Token {
    Open,
    Close,
    Integer(u32),
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

fn parse_tokens(line: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut digit_cache: String = "".to_string();
    for new_char in line.chars() {
        if new_char.is_ascii_digit() {
            digit_cache.push(new_char); // keep track of consecutive digits
        } else if !digit_cache.is_empty() {
            // convert string of cached digits to u32
            tokens.push(Token::Integer(digit_cache.parse::<u32>().unwrap()));
            digit_cache = "".to_string();
        }
        if new_char == '[' {
            tokens.push(Token::Open)
        } else if new_char == ']' {
            tokens.push(Token::Close)
        }
    }
    tokens
}

fn parse_packet(tokens: Vec<Token>) -> (Packet, Vec<Token>) {
    let t = &tokens[0];
    match t {
        Token::Open => {
            parse_list(tokens[1..].to_vec()) // parse the list body
        }
        Token::Integer(value) => {
            (Packet::Integer(*value), tokens[1..].to_vec()) // return int value and remaining tokens
        }
        _ => panic!("close brackets should be parsed in parse_list"),
    }
}

fn parse_list(mut tokens: Vec<Token>) -> (Packet, Vec<Token>) {
    let t = &tokens[0];
    let mut list = Vec::<Packet>::new();
    match t {
        Token::Close => (Packet::List(list), tokens[1..].to_vec()), // empty list
        _ => {
            while !matches!(tokens[0], Token::Close) {
                // keep parsing tokens until next closed bracket
                let (pack, new_tokens) = parse_packet(tokens.to_vec());
                tokens = new_tokens;
                list.push(pack);
            }
            (Packet::List(list.clone()), tokens[1..].to_vec())
        }
    }
}

fn parse_input_p1(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n") // each group of two packets
        .map(|pair| {
            pair.lines() // each packet line as str
                .map(|l| parse_packet(parse_tokens(l)).0) // str to tokens to Packet
                .collect_tuple::<(Packet, Packet)>()
                .unwrap()
        })
        .collect_vec()
}

fn parse_input_p2(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(parse_tokens(l)).0) // str to tokens to Packet
        .collect_vec()
}

fn compare_list(l_list: &Vec<Packet>, r_list: &Vec<Packet>) -> Option<bool> {
    for (left, right) in zip(l_list, r_list) {
        if let Some(correct) = compare(left, right) {
            return Some(correct);
        }
    }
    // ran out of items
    match l_list.cmp(r_list) {
        Ordering::Less => Some(true), // all equal and left shorter is correct
        Ordering::Greater => Some(false), // all equal but right shorter is wrong
        _ => None,                    // all equal and same length is unknown, continue
    }
}

fn compare(left: &Packet, right: &Packet) -> Option<bool> {
    match left {
        Packet::Integer(l_value) => match right {
            Packet::Integer(r_value) => {
                match l_value.cmp(r_value) {
                    Ordering::Greater => Some(false), // left larger than right is wrong
                    Ordering::Less => Some(true),     // left smaller than right is correct
                    Ordering::Equal => None, // if left and right are equal we don't know yet, continue
                }
            }
            Packet::List(r_list) => {
                let l_list = &vec![Packet::Integer(*l_value)];
                compare_list(l_list, r_list)
            }
        },
        Packet::List(l_list) => match right {
            Packet::Integer(r_value) => {
                let r_list = &vec![Packet::Integer(*r_value)];
                compare_list(l_list, r_list)
            }
            Packet::List(r_list) => compare_list(l_list, r_list),
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = parse_input_p1(input);
    let mut sum_idx: usize = 0;
    for (idx, p) in pairs.iter().enumerate() {
        match compare(&p.0, &p.1) {
            Some(true) => {
                sum_idx += idx + 1;
            }
            Some(false) => {
                continue;
            }
            _ => {
                panic!("undefined comparison");
            }
        }
    }
    Some(sum_idx as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = parse_input_p2(input);
    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort_by(|l, r| match compare(l, r) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        _ => Ordering::Equal,
    });
    let div1_idx = packets.iter().position(|pack| *pack == divider_1).unwrap() + 1;
    let div2_idx = packets.iter().position(|pack| *pack == divider_2).unwrap() + 1;

    Some((div1_idx * div2_idx) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
