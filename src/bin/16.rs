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

/// recursively compute valve opening order with the best total pressure
fn run(
    flows: &Vec<u32>,          // pressure of each valve (when open)
    distances: &Vec<Vec<u32>>, // distance from each valve to each other valve
    time: u32,                 // current time
    end: u32,                  // end time
    open_valves: Vec<usize>,   // valves that have been turned on so far
    current_pressure: u32, // if no more valves were turned on, total pressure achieved at end time
    mut best_pressure: u32, // best total pressure achieved so far
) -> u32 {
    // candidate valve to visit next
    for valve in 0..flows.len() {
        // have we already opened this valve?
        if !open_valves.contains(&valve) {
            // time taken to open valve is distance to it +1
            let new_t = time + distances[open_valves[open_valves.len() - 1]][valve] + 1;

            // would opening this valve take us over the time limit?
            if new_t < end {
                // add pressure released in remaining time from opening this valve, and check if this is a new best
                let new_pressure = current_pressure + (end - new_t) * flows[valve];
                let new_best = if new_pressure > best_pressure {
                    new_pressure
                } else {
                    best_pressure
                };

                // run again with updated values (after adding new valve to the list of opened ones)
                let mut new_valves = open_valves.clone();
                new_valves.push(valve);
                best_pressure = run(
                    flows,
                    distances,
                    new_t,
                    end,
                    new_valves,
                    new_pressure,
                    new_best,
                );
            }
        }
    }
    best_pressure
}

/// recursively compute valve opening order with the best total pressure
/// nansty copy/pasete of p1 with additionanl loop over current position/time for
/// one of N agents
/// Slow - took >2hrs to run.
fn run_p2(
    flows: &Vec<u32>,          // pressure of each valve (when open)
    distances: &Vec<Vec<u32>>, // distance from each valve to each other valve
    times: Vec<u32>,           // current time for each agent
    positions: Vec<usize>,     // most recently visited valve for each agent
    end: u32,                  // end time
    remaining_valves: Vec<usize>,   // valves that have NOT been turned on so far
    current_pressure: u32, // if no more valves were turned on, total pressure achieved at end time
    mut best_pressure: u32, // best total pressure achieved so far
) -> u32 {
    // candidate valve to visit next
    for valve in &remaining_valves {
        for agent_idx in 0..positions.len() {
            // time taken to open valve is distance to it +1
            let new_t = times[agent_idx] + distances[positions[agent_idx]][*valve] + 1;

            // would opening this valve take us over the time limit?
            if new_t < end {
                // add pressure released in remaining time from opening this valve, and check if this is a new best
                let new_pressure = current_pressure + (end - new_t) * flows[*valve];
                let new_best = if new_pressure > best_pressure {
                    new_pressure
                } else {
                    best_pressure
                };

                // run again with updated values (after adding new valve to the list of opened ones)
                let mut new_valves = remaining_valves.clone();
                new_valves.retain(|v| *v != *valve);
                let mut new_positions = positions.clone();
                new_positions[agent_idx] = *valve;
                let mut new_times = times.clone();
                new_times[agent_idx] = new_t;

                best_pressure = run_p2(
                    flows,
                    distances,
                    new_times,
                    new_positions,
                    end,
                    new_valves.to_vec(),
                    new_pressure,
                    new_best,
                );
            }
        }
    }
    best_pressure
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_valves = parse_valves(input);
    let (good_valves, distances) = path_lengths(&all_valves);
    let flows = good_valves
        .iter()
        .map(|v| all_valves.get(v).unwrap().flow_rate)
        .collect_vec();
    let start_idx = good_valves.iter().position(|v| v == "AA").unwrap();

    let best_total = run(&flows, &distances, 0, 30, vec![start_idx], 0, 0);
    Some(best_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_valves = parse_valves(input);
    let (good_valves, distances) = path_lengths(&all_valves);
    let flows = good_valves
        .iter()
        .map(|v| all_valves.get(v).unwrap().flow_rate)
        .collect_vec();
    let start_idx = good_valves.iter().position(|v| v == "AA").unwrap();

    // possible next valves (remaining) = all valves except the start one AA, whih always has 0 flow
    let mut remaining_valves = (0..flows.len()).collect_vec();
    remaining_valves.retain(|v| *v != start_idx);

    let best_total = run_p2(
        &flows,
        &distances,
        vec![0, 0],
        vec![start_idx, start_idx],
        26,
        remaining_valves,
        0,
        0,
    );
    Some(best_total)
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
