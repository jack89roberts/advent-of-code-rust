use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<String>, // other valves this valve connects to
}

fn parse_valves(input: &str) -> HashMap<String, Valve> {
    let re =
        Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    input
        .lines()
        .map(|line| {
            let matches = &re.captures_iter(line).collect_vec()[0];
            let tunnel_name = matches[1].to_string();
            let flow_rate = matches[2].parse::<u32>().unwrap();
            let tunnels = matches[3]
                .split(", ")
                .map(|label| label.to_string())
                .collect_vec();
            (tunnel_name, Valve { flow_rate, tunnels })
        })
        .collect()
}

/// get path lengths between all the non-zero nodes (+ the start node)
fn path_lengths(valves: &HashMap<String, Valve>) -> (Vec<String>, Vec<Vec<u32>>) {
    // only compute paths for the start valve (AA) and valves with non-zero flow rate
    let good_valves = valves
        .keys()
        .filter(|k| (k.to_string() == *"AA") | (valves.get(&k.to_string()).unwrap().flow_rate > 0))
        .map(|s| s.to_string())
        .collect_vec();

    // distance from each interesting valve to each other interesting valve
    let mut distances = vec![vec![u32::MAX; good_valves.len()]; good_valves.len()];
    for (idx_start, start) in good_valves.iter().enumerate() {
        for (idx_end, end) in good_valves.iter().enumerate() {
            let result: (Vec<String>, usize) = dijkstra(
                start,
                |v| {
                    valves
                        .get(v)
                        .unwrap()
                        .tunnels
                        .iter()
                        .map(|tunnel| (tunnel.to_string(), 1)) // add cost of 1 minute to get to neighbouring valves
                        .collect::<Vec<(String, usize)>>()
                },
                |v| *v == *end,
            )
            .unwrap();
            distances[idx_start][idx_end] = result.1 as u32;
        }
    }
    (good_valves, distances)
}

fn run_p1(
    flows: &Vec<u32>,
    distances: &Vec<Vec<u32>>,
    time: u32,
    end: u32,
    path: Vec<usize>,
    sum: u32,
    mut best: u32,
) -> u32 {
    for valve in 0..flows.len() {
        if !path.contains(&valve) {
            let new_t = time + distances[path[path.len() - 1]][valve] + 1; // time taken to open valve is distance to it +1
            if new_t < end {
                let new_sum = sum + (end - new_t) * flows[valve];
                let new_best = if new_sum > best { new_sum } else { best };
                let mut new_path = path.clone();
                new_path.push(valve);
                best = run_p1(flows, distances, new_t, end, new_path, new_sum, new_best);
            }
        }
    }
    best
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_valves = parse_valves(input);
    let (good_valves, distances) = path_lengths(&all_valves);
    let flows = good_valves
        .iter()
        .map(|v| all_valves.get(v).unwrap().flow_rate)
        .collect_vec();
    let start_idx = good_valves.iter().position(|v| v == "AA").unwrap();

    let best_total = run_p1(&flows, &distances, 0, 30, vec![start_idx], 0, 0);
    Some(best_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
