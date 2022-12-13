use itertools::Itertools;

#[derive(Debug, Clone)]
enum Token {
    Open,
    Close,
    Integer(u32),
}

#[derive(Debug, Clone)]

enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}


fn parse_tokens(line: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut digit_cache: String = "".to_string();
    for new_char in line.chars() {
        if new_char.to_digit(10).is_some() { // is a digit
            digit_cache.push(new_char); // keep track of consecutive digits
        } else if digit_cache.len() > 0 { // convert string of cached digits to u32
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
            parse_list(tokens[1..].to_vec())
        },
        Token::Integer(value) => {
            (Packet::Integer(*value), tokens[1..].to_vec())
        },
        _ => panic!("close brackets should be parsed elsewhere")
    }
}

fn parse_list(mut tokens: Vec<Token>) -> (Packet, Vec<Token>) {
    let t = &tokens[0];
    let mut list = Vec::<Packet>::new();
    match t {
        Token::Close => (Packet::List(list), tokens[1..].to_vec()),
        _ => {
            while !matches!(tokens[0], Token::Close) {
                let (pack, new_tokens) = parse_packet(tokens.to_vec());
                tokens = new_tokens.to_vec();
                list.push(pack);
            }
            return (Packet::List(list.clone()), tokens[1..].to_vec());
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .split("\n\n")
        .map(|pair| pair.lines().collect_tuple::<(&str, &str)>().unwrap())
        .collect_vec();
    let mut sum_ok_indices: u32 = 0;
    for (left, right) in pairs {
        let left_tokens = parse_tokens(left);
        let right_tokens = parse_tokens(right);
        println!("LEFT  {:?}", parse_packet(left_tokens).0);
        println!("RIGHT {:?}\n", parse_packet(right_tokens).0);

    }
    Some(sum_ok_indices)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
