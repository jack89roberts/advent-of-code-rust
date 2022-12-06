use std::collections::HashSet;

fn find_consecutive_unique(n: usize, input: &str) -> Option<u32> {
    let mut marker: u32 = 0;
    for slice in input.chars().collect::<Vec<char>>().windows(n) {
        if HashSet::<&char>::from_iter(slice).len() == n {
            break;
        }
        marker += 1;
    }
    marker += n as u32; // move marker from start of sequence to end
    Some(marker)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_consecutive_unique(4, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_consecutive_unique(14, input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
