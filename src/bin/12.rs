use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Index(usize, usize);
struct Node {
    edges: Vec<Index>, // position and edge weight (always 1)
}
impl Node {
    fn neighbours(&self) -> Vec<(Index, usize)> {
        self.edges.clone().into_iter().map(|e| (e, 1)).collect()
    }
}

fn parse_input(input: &str, part2: bool) -> (Vec<Index>, Index, HashMap<Index, Node>) {
    let labels = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    // find start and target idx
    let mut start = Vec::<Index>::new();
    let mut target = Index(usize::MAX, usize::MAX);
    for i in 0..labels.len() {
        for j in 0..labels[0].len() {
            if labels[i][j] == 'S' {
                start.push(Index(i, j));
            }
            // if part2 consider all elevation 'a' locations as start candidates
            if part2 & (labels[i][j] == 'a') {
                start.push(Index(i, j))
            }
            if labels[i][j] == 'E' {
                target = Index(i, j);
            }
        }
    }

    // determine elevations from labels
    let elevations = labels
        .iter()
        .map(|line| {
            line.iter()
                .map({
                    |c| match c {
                        'S' => 0,              // start position at elevation 'a'
                        'E' => 25,             // target position at elevation 'z'
                        _ => (*c as u32) - 97, // char to alphabet idx
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    // this mess makes a HashMap of node index to vec of node indices it connects to
    let mut nodes: HashMap<Index, Node> = HashMap::new();
    for i in 0..labels.len() {
        for j in 0..labels[0].len() {
            nodes.insert(
                Index(i, j),
                Node {
                    edges: Vec::<Index>::new(),
                },
            );
            for i_delta in -1i32..2 {
                // check row delta doesn't take us outside the grid
                if (i as i32 + i_delta < 0) | (i as i32 + i_delta >= labels.len() as i32) {
                    continue;
                }
                for j_delta in -1i32..2 {
                    // check this isn't a diagonal move
                    if i_delta.abs() + j_delta.abs() > 1 {
                        continue;
                    }
                    // check we aren't comparing to ourself
                    if (i_delta == 0) & (j_delta == 0) {
                        continue;
                    }
                    // check column delta doesn't take us outside the grid
                    if (j as i32 + j_delta < 0) | (j as i32 + j_delta >= labels[0].len() as i32) {
                        continue;
                    }
                    // if this node is less than one higher than us we can step there
                    if (elevations[(i as i32 + i_delta) as usize][(j as i32 + j_delta) as usize]
                        as i32)
                        - (elevations[i][j] as i32)
                        <= 1
                    {
                        nodes.get_mut(&Index(i, j)).unwrap().edges.push(Index(
                            (i as i32 + i_delta) as usize,
                            (j as i32 + j_delta) as usize,
                        ));
                    }
                }
            }
        }
    }
    (start, target, nodes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, target, nodes) = parse_input(input, false);
    let result = dijkstra(
        &start[0],
        |node| nodes.get(node).unwrap().neighbours(),
        |idx| *idx == target,
    )
    .unwrap();
    Some(result.1 as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_candidates, target, nodes) = parse_input(input, true);
    let mut min_path = u32::MAX;
    for start in start_candidates {
        let result = dijkstra(
            &start,
            |node| nodes.get(node).unwrap().neighbours(),
            |idx| *idx == target,
        );
        if let Some(result) = result {
            let length = result.1 as u32;
            if length < min_path {
                min_path = length;
            }
        }
    }
    Some(min_path)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
