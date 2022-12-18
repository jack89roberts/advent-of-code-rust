use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cave = Cave::new(input);
    let n_rocks = 2022;
    for _ in 0..n_rocks {
        cave.step();
    }
    Some(cave.height())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cave = Cave::new(input);
    let n_rocks = 10000; // initial rocks to simulate to find a pattern
    for _ in 0..n_rocks {
        cave.step();
    }

    // search for a repeating pattern -> two equal states, where a state contains
    // the next shape that will appear, the index in the jet directions, and the height
    // to the topmost shape in each column
    let mut pattern_start = 0;
    let mut pattern_end = 0;
    'outer: for left in 0..(n_rocks - 1) {
        for right in (left + 1..n_rocks).rev() {
            if cave.states[left] == cave.states[right] {
                println!("{left} to {right}");
                pattern_start = left;
                pattern_end = right;
                break 'outer;
            }
        }
    }

    // compute height from number of repeating cycles
    let n_rocks: usize = 1000000000000;
    let pattern_len = pattern_end - pattern_start;
    let cycles = (n_rocks - pattern_start) / pattern_len;
    let remaining = (n_rocks - pattern_start) % pattern_len;

    let height_per_cycle = cave.heights[pattern_end - 1] - cave.heights[pattern_start - 1];
    let height_start = cave.heights[pattern_start - 1];
    let height_end = cave.heights[pattern_start + remaining - 1] - cave.heights[pattern_start - 1];

    Some(height_start + (height_per_cycle * cycles) + height_end)
}

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

    fn step(&self, dir: Direction, n: i64) -> Rock {
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

struct Cave {
    rock_types: Vec<Rock>,
    jet_directions: Vec<Direction>,
    width: usize,
    rocks: Vec<Rock>,
    jet_idx: usize,
    type_idx: usize,
    states: Vec<State>,
    heights: Vec<usize>,
}
impl Cave {
    fn new(input: &str) -> Cave {
        Cave {
            rock_types: get_rock_types(),
            jet_directions: get_jet_directions(input),
            width: 7,
            rocks: Vec::<Rock>::new(),
            jet_idx: 0,
            type_idx: 0,
            states: Vec::<State>::new(),
            heights: Vec::<usize>::new(),
        }
    }

    fn step(&mut self) {
        let height = if self.rocks.is_empty() {
            3
        } else {
            self.rocks.iter().max_by_key(|r| r.top()).unwrap().top() + 4
        };

        let mut new_rock = self.rock_types[self.type_idx].shift_y(height);
        let mut rock_placed = false;
        let mut jet_move = true;
        while !rock_placed {
            let direction = match jet_move {
                true => self.jet_directions[self.jet_idx],
                false => Direction::Down,
            };

            if (new_rock.base() == 0) & (direction == Direction::Down) {
                // rock will collide with floor and stop moving
                rock_placed = true;
            } else if !(((new_rock.left() == 0) & (direction == Direction::Left))
                | ((new_rock.right() == self.width as i64 - 1) & (direction == Direction::Right)))
            {
                // horrific long condition means: if rock will NOT collide with the walls on this step
                // will it collide with any other rocks?
                let mut next_step = new_rock.step(direction, 1);

                for r in self.rocks.iter().rev() {
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
                self.jet_idx = (self.jet_idx + 1) % self.jet_directions.len();
            }
        }

        // add the new rock and update the next step
        self.rocks.push(new_rock);
        self.type_idx = (self.type_idx + 1) % self.rock_types.len();

        self.heights.push(self.height());
        self.states.push(self.get_state());
    }

    fn height(&self) -> usize {
        // max y value of all placed rocks
        (self.rocks.iter().max_by_key(|r| r.top()).unwrap().top() + 1) as usize
    }

    fn get_state(&self) -> State {
        let max_height = self.height();
        let mut col_heights = [0; 7];
        for (idx, x) in (0..self.width).enumerate() {
            col_heights[idx] = max_height
                - self
                    .rocks
                    .iter()
                    .map(|r| {
                        r.points
                            .iter()
                            .filter(|p| p.x == x as i64)
                            .max_by_key(|p| p.y)
                            .unwrap_or(&Point { x: x as i64, y: 0 })
                            .y
                    })
                    .max()
                    .unwrap() as usize;
        }
        State {
            col_heights,
            jet_idx: self.jet_idx,
            type_idx: self.type_idx,
        }
    }
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, Debug)]
struct State {
    col_heights: [usize; 7],
    jet_idx: usize,
    type_idx: usize,
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
