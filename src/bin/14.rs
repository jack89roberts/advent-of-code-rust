use std::cmp::Ordering;
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

/// convert vec of points (connected by arrows) to HashSet of all points included in that path
fn expand_path(instruction: &[Point]) -> HashSet<Point> {
    let mut path = HashSet::<Point>::from_iter(instruction.to_owned());
    for slice in instruction.windows(2) {
        let start = slice[0];
        let end = slice[1];
        match start.x.cmp(&end.x) {
            Ordering::Equal => {
                // horizontal path
                let ys = vec![start.y, end.y];
                let ymin = *ys.iter().min().unwrap();
                let ymax = *ys.iter().max().unwrap();
                for y in ymin..(ymax + 1) {
                    path.insert(Point { x: start.x, y });
                }
            }
            _ => {
                // vertical path
                let xs = vec![start.x, end.x];
                let xmin = *xs.iter().min().unwrap();
                let xmax = *xs.iter().max().unwrap();
                for x in xmin..(xmax + 1) {
                    path.insert(Point { x, y: start.y });
                }
            }
        }
    }
    path
}

/// input -> list of points that are blocked
fn parse_input(input: &str) -> HashSet<Point> {
    let instructions = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let xy = coords.split(',').collect_tuple::<(&str, &str)>().unwrap();
                    Point {
                        x: xy.0.parse::<u32>().unwrap(),
                        y: xy.1.parse::<u32>().unwrap(),
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut blocked = HashSet::<Point>::new();
    for instr in instructions {
        blocked.extend(expand_path(&instr));
    }
    blocked
}

fn drop_sand(blocked: &HashSet<Point>, abyss: Point, part2: bool) -> Option<Point> {
    let mut pos = Point { x: 500, y: 0 }; // current position of sand
    loop {
        if !part2 & ((pos.x <= abyss.x) | (pos.x >= abyss.x)) & (pos.y >= abyss.y) {
            return None; // reached the abyss without settling
        } else if part2 & (pos.y > abyss.y) {
            return Some(pos);
        } else if !blocked.contains(&Point {
            x: pos.x,
            y: pos.y + 1,
        }) {
            pos.y += 1; // move down
        } else if !blocked.contains(&Point {
            x: pos.x - 1,
            y: pos.y + 1,
        }) {
            pos.y += 1; // move down
            pos.x -= 1; // move left
        } else if !blocked.contains(&Point {
            x: pos.x + 1,
            y: pos.y + 1,
        }) {
            pos.y += 1; // move down
            pos.x += 1; // move right
        } else {
            return Some(pos); // nowhere to move, settled
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blocked = parse_input(input);
    let abyss = Point {
        x: blocked.iter().min_by_key(|p| p.x).unwrap().x,
        y: blocked.iter().max_by_key(|p| p.y).unwrap().y,
    };
    let mut n_sand = 0;
    loop {
        let settled = drop_sand(&blocked, abyss, false);
        match settled {
            Some(point) => {
                n_sand += 1;
                blocked.insert(point);
            }
            _ => {
                break;
            }
        }
    }
    Some(n_sand)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut blocked = parse_input(input);
    let abyss = Point {
        x: blocked.iter().min_by_key(|p| p.x).unwrap().x,
        y: blocked.iter().max_by_key(|p| p.y).unwrap().y,
    };
    let mut n_sand = 0;
    loop {
        let settled = drop_sand(&blocked, abyss, true);
        match settled {
            Some(point) => {
                n_sand += 1;
                if point == (Point { x: 500, y: 0 }) {
                    break;
                } else {
                    blocked.insert(point);
                }
            }
            _ => {
                break;
            }
        }
    }
    Some(n_sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
