use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn step(&self, dir: Direction, n: i64) -> Point {
        match dir {
            Direction::Left => Point {
                x: self.x - n,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + n,
                y: self.y,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y + n,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - n,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Rock {
    points: Vec<Point>,
}
impl Rock {
    fn base(&self) -> i64 {
        // lowest y value
        self.points.iter().min_by_key(|p| p.y).unwrap().y
    }

    fn top(&self) -> i64 {
        // max y value
        self.points.iter().max_by_key(|p| p.y).unwrap().y
    }

    fn left(&self) -> i64 {
        // lowest x value
        self.points.iter().min_by_key(|p| p.x).unwrap().x
    }

    fn right(&self) -> i64 {
        // max x value
        self.points.iter().max_by_key(|p| p.x).unwrap().x
    }

    fn shift_y(&self, y: i64) -> Rock {
        // shift rock so base is at y
        let n: i64 = (y as i64) - (self.base() as i64);
        if n > 0 {
            self.step(Direction::Up, n)
        } else {
            self.step(Direction::Down, n)
        }
    }

    fn step(&self, dir: Direction, n: u64) -> Rock {
        Rock {
            points: self.points.iter().map(|p| p.step(dir, n)).collect_vec(),
        }
    }
    fn collides(&self, other: &Rock) -> bool {
        if (other.base() > self.top()) | (self.base() > other.top()) {
            return false;
        }
        if (other.left() > self.right()) | (self.left() > other.right()) {
            return false;
        }
        for me in &self.points {
            for you in &other.points {
                if me == you {
                    return true;
                }
            }
        }
        false
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut text: String = String::new();
        for y in (self.base()..(self.top() + 1)).rev() {
            for x in self.left()..(self.right() + 1) {
                if self.points.contains(&Point { x, y }) {
                    text.push('#')
                } else {
                    text.push('.')
                }
            }
            text.push('\n');
        }
        write!(f, "{text}")
    }
}

fn get_rock_types() -> Vec<Rock> {
    vec![
        Rock {
            // -
            points: vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 5, y: 3 },
            ],
        },
        Rock {
            // +
            points: vec![
                Point { x: 3, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 4 },
                Point { x: 3, y: 5 },
            ],
        },
        Rock {
            // L
            points: vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 4 },
                Point { x: 4, y: 5 },
            ],
        },
        Rock {
            // |
            points: vec![
                Point { x: 2, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 6 },
            ],
        },
        Rock {
            // ■
            points: vec![
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 3, y: 4 },
            ],
        },
    ]
}

fn get_jet_directions(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("unexpected direction {c}"),
        })
        .collect_vec()
}

fn move_floor(rocks: &Vec<Rock>, width: u64) -> (Vec<Rock>, Option<i64>) {
    let max_y = rocks.iter().max_by_key(|r| r.top()).unwrap().top();

    let mut floor_y: Option<i64> = None;
    'outer: for y in (0..max_y + 1).rev() {
        let mut xs = HashSet::<i64>::new();
        for r in rocks {
            for p in &r.points {
                if p.y == y {
                    xs.insert(p.x);
                    if (xs.len() as i64) == width {
                        floor_y = Some(y);
                        break 'outer;
                    }
                }
            }
        }
    }

    match floor_y {
        Some(y) => {

        }
        _ => (*rocks, None)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rock_types = get_rock_types();
    let jet_directions = get_jet_directions(input);
    let n_rocks: u64 = 100;
    let width: u64 = 7;

    let mut jet_idx = 0;
    let mut type_idx: usize = 0;
    let mut rocks = Vec::<Rock>::new();
    for _ in 0..n_rocks {
        // current height of the rock tower + 3 (height new rock appears)
        let height = if rocks.is_empty() {
            3
        } else {
            rocks.iter().max_by_key(|r| r.top()).unwrap().top() + 4
        };

        let mut new_rock = rock_types[type_idx].shift_y(height);
        let mut rock_placed = false;
        let mut jet_move = true;

        while !rock_placed {
            let direction = match jet_move {
                true => jet_directions[jet_idx],
                false => Direction::Down,
            };

            if (new_rock.base() == 0) & (direction == Direction::Down) {
                // rock will collide with floor and stop moving
                rock_placed = true;
            } else if !(((new_rock.left() == 0) & (direction == Direction::Left))
                | ((new_rock.right() == width - 1) & (direction == Direction::Right)))
            {
                // horrific long condition means: if rock will NOT collide with the walls on this step
                // will it collide with any other rocks?
                let mut next_step = new_rock.step(direction, 1);

                for r in rocks.iter().rev() {
                    // reverse order above as more likely to collide with recently placed rocks
                    if next_step.collides(r) {
                        if direction == Direction::Down {
                            // collide with another rock whilst moving down means movement stops
                            rock_placed = true;
                        }
                        // a collision occurred so reset this step
                        next_step = new_rock.clone();
                        break;
                    }
                }
                new_rock = next_step;
            }

            // update the direction indicators for next iteration
            jet_move = !jet_move;
            if jet_move {
                jet_idx = (jet_idx + 1) % jet_directions.len();
            }
        }

        // add the new rock and update the next step
        rocks.push(new_rock);
        type_idx = (type_idx + 1) % rock_types.len();
        //println!("jet {jet_idx} type {type_idx}");
        //if (jet_idx == 0) {//& (type_idx == 0) {

    }

    // max y value of all placed rocks
    let result = rocks.iter().max_by_key(|r| r.top()).unwrap().top() + 1;
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
