use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

fn is_adjacent(h_pos: (i32, i32), t_pos: (i32, i32)) -> bool {
    if ((h_pos.0 - t_pos.0).abs() <= 1) & ((h_pos.1 - t_pos.1).abs() <= 1) {
        return true;
    }
    false
}

fn move_tail(h_pos: (i32, i32), t_pos: (i32, i32)) -> (i32, i32) {
    if !is_adjacent(h_pos, t_pos) {
        (
            t_pos.0
                + match h_pos.0.cmp(&t_pos.0) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                },
            t_pos.1
                + match h_pos.1.cmp(&t_pos.1) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                },
        )
    } else {
        t_pos
    }
}

fn move_rope(pos: &mut Vec<(i32, i32)>, dir: &str, n: u32, visited: &mut [HashSet<(i32, i32)>]) {
    let delta = match dir {
        "U" => (-1, 0),
        "D" => (1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => panic!("unknown direction"),
    };
    for _ in 0..n {
        // move head
        pos[0] = (pos[0].0 + delta.0, pos[0].1 + delta.1);
        for knot in 1..pos.len() {
            (pos[knot].0, pos[knot].1) = move_tail(pos[knot - 1], pos[knot]);
            visited[knot].insert(pos[knot]);
        }
    }
}

fn count_tail_visited(input: &str, knots: u32) -> u32 {
    let mut pos = vec![(0, 0); knots as usize];
    let mut visited = vec![HashSet::<(i32, i32)>::new(); knots as usize];
    for instr in input.lines() {
        let parts = instr.split(' ').collect_vec();
        let dir = parts[0];
        let n = parts[1].parse::<u32>().unwrap();
        move_rope(&mut pos, dir, n, &mut visited);
    }
    visited[visited.len() - 1].len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(count_tail_visited(input, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(count_tail_visited(input, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));

        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("src").join("examples").join("09b.txt");
        let input2 = fs::read_to_string(filepath).unwrap();
        assert_eq!(part_two(&input2), Some(36));
    }
}
