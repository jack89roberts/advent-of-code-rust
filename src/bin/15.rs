use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn manhatten(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i32,
    max: i32,
}
impl Range {
    fn extend(&mut self, other: &Range) -> (Range, Option<Range>) {
        if (other.max < self.min - 1) | (other.min > self.max + 1) {
            // no overlap between self and other
            (*self, Some(*other))
        } else if (other.min >= self.min) & (other.max <= self.max) {
            //  self completely encompasses other
            (*self, None)
        } else {
            // extend range
            let min = *vec![self.min, other.min].iter().min().unwrap();
            let max = *vec![self.max, other.max].iter().max().unwrap();
            (Range { min, max }, None)
        }
    }
    fn contains(&self, other: i32) -> bool {
        (other >= self.min) & (other <= self.max)
    }
    fn len(&self) -> u32 {
        (self.max - self.min + 1) as u32
    }
    fn clip(&self, extent: Range) -> Range {
        let new_min = match self.min.cmp(&extent.min) {
            Ordering::Less => extent.min,
            _ => self.min,
        };
        let new_max = match self.max.cmp(&extent.max) {
            Ordering::Greater => extent.max,
            _ => self.max,
        };
        Range {
            min: new_min,
            max: new_max,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Reading {
    sensor: Point,
    beacon: Point,
}
impl Reading {
    fn radius(&self) -> i32 {
        // max horizontal/vertical distance sensor covers (manhatten distanace to beaacon)
        self.sensor.manhatten(&self.beacon)
    }
    fn range(&self, y: i32) -> Option<Range> {
        // get Range of x coordinates where beacons cannot be present due to this reading
        let delta = y - self.sensor.y;
        let width = self.radius() - delta.abs();
        if width < 0 {
            None // no coverage from this sensor on this row
        } else {
            Some(Range {
                min: self.sensor.x - width,
                max: self.sensor.x + width,
            })
        }
    }
}

fn parse_input(input: &str) -> Vec<Reading> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();

    let pairs = input
        .lines()
        .map(|line| {
            line.split(": ")
                .map(|part| {
                    let matches = &re.captures_iter(part).collect_vec()[0];
                    Point {
                        x: matches[1].parse::<i32>().unwrap(),
                        y: matches[2].parse::<i32>().unwrap(),
                    }
                })
                .collect_tuple::<(Point, Point)>()
                .unwrap()
        })
        .collect_vec();

    pairs
        .iter()
        .map(|(sensor, beacon)| Reading {
            sensor: *sensor,
            beacon: *beacon,
        })
        .collect_vec()
}

fn merge_ranges(mut ranges: Vec<Option<Range>>) -> Vec<Option<Range>> {
    let mut merged = vec![ranges[0]];
    for new in ranges.iter_mut().skip(1) {
        for existing in &mut merged {
            if new.is_none() {
                break;
            }
            let (extended, remaining) = existing.unwrap().extend(&new.unwrap());
            *existing = Some(extended);
            *new = remaining;
        }
        if new.is_some() {
            merged.push(*new);
        }
    }
    merged
}

fn get_row_ranges(readings: &[Reading], y: i32) -> Vec<Range> {
    // range of cells that each sensor covers on row y
    let mut ranges = readings
        .iter()
        .map(|r| r.range(y))
        .filter(|r| r.is_some()) // remove empty ranges
        .collect_vec();

    // combine overlapping ranges
    let mut n_ranges = 0;
    while n_ranges != ranges.len() {
        n_ranges = ranges.len();
        ranges = merge_ranges(ranges);
    }
    ranges.iter().map(|r| r.unwrap()).collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let readings = parse_input(input);
    let y = match readings.len().cmp(&14) {
        // decide whether to use y test or y actual
        Ordering::Equal => 10, // test input
        _ => 2000000,          // actual input
    };
    let ranges = get_row_ranges(&readings, y);

    // check for (unique) beacons at the observed coordinates
    let beacons = readings
        .iter()
        .map(|r| r.beacon)
        .filter(|b| b.y == y)
        .collect::<HashSet<Point>>();
    let mut seen_beacons: u32 = 0;
    for b in beacons {
        for r in &ranges {
            if r.contains(b.x) {
                seen_beacons += 1;
                break;
            }
        }
    }

    // count excluded cells
    let range_excluded = ranges.iter().fold(0, |acc, r| acc + r.len()); // no. cells in all ranges
    Some(range_excluded - seen_beacons)
}

pub fn part_two(input: &str) -> Option<u64> {
    let readings = parse_input(input);
    let extent = match readings.len().cmp(&14) {
        Ordering::Equal => Range { min: 0, max: 20 }, // test input
        _ => Range {
            min: 0,
            max: 4000000,
        }, // actual input
    };
    for y in extent.min..(extent.max + 1) {
        // get seen cells (ranges) for this row and clip them to the extent
        let mut ranges = get_row_ranges(&readings, y)
            .iter()
            .map(|r| Some(r.clip(extent)))
            .collect_vec();

        // combine overlapping ranges
        let mut n_ranges = 0;
        while n_ranges != ranges.len() {
            n_ranges = ranges.len();
            ranges = merge_ranges(ranges);
        }
        if ranges.len() == 2 {
            // found an unseen spot (2 ranges with a gap between)
            let left = ranges[0].unwrap();
            let right = ranges[1].unwrap();
            let coord = match right.min.cmp(&left.max) {
                Ordering::Greater => Point { x: left.max + 1, y },
                _ => Point {
                    x: right.max + 1,
                    y,
                },
            };
            return Some(((coord.x as u64) * 4000000) + (coord.y as u64));
        }
    }
    panic!("Didn't find space for beacon!")
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
