use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, n_cubes) = parse_grid(input);
    let mut side_count: usize = n_cubes * 6;

    // check for shared edges
    let xlen = grid.len();
    let ylen = grid[0].len();
    let zlen = grid[0][0].len();
    for x in 0..xlen {
        for y in 0..ylen {
            for z in 0..zlen {
                if grid[x][y][z] {
                    side_count -= shared_edges(x, y, z, &grid);
                }
            }
        }
    }
    Some(side_count as u32)
}

#[allow(clippy::needless_range_loop)]
pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _) = parse_grid(input);
    let xlen = grid.len();
    let ylen = grid[0].len();
    let zlen = grid[0][0].len();

    let mut wet = vec![vec![vec![false; zlen]; ylen]; xlen];
    let mut queue = VecDeque::<Point>::new();
    queue.push_front(Point { x: 0, y: 0, z: 0 });

    wet = bfs(&grid, &mut queue, &mut wet);

    // check for edges hit by water
    let mut side_count = 0;
    for x in 0..xlen {
        for y in 0..ylen {
            for z in 0..zlen {
                if wet[x][y][z] {
                    side_count += shared_edges(x, y, z, &grid);
                }
            }
        }
    }
    Some(side_count as u32)
}

fn shared_edges(x: usize, y: usize, z: usize, grid: &Vec<Vec<Vec<bool>>>) -> usize {
    // how many neighbouring cubes to x,y,z?
    let xlen = grid.len();
    let ylen = grid[0].len();
    let zlen = grid[0][0].len();
    let mut shared_edges = 0;

    for xdelta in [-1, 1] {
        if ((x as i32) + xdelta < 0) | ((x as i32) + xdelta >= xlen as i32) {
            continue; // don't go outside of grid
        }
        if grid[(x as i32 + xdelta) as usize][y][z] {
            //  shares an edge
            shared_edges += 1;
        }
    }
    for ydelta in [-1, 1] {
        if ((y as i32) + ydelta < 0) | ((y as i32) + ydelta >= ylen as i32) {
            continue; // don't go outside of grid
        }
        if grid[x][((y as i32) + ydelta) as usize][z] {
            //  shares an edge
            shared_edges += 1;
        }
    }
    for zdelta in [-1, 1] {
        if ((z as i32) + zdelta < 0) | ((z as i32) + zdelta >= zlen as i32) {
            continue; // don't go outside of grid
        }
        if grid[x][y][(z as i32 + zdelta) as usize] {
            //  shares an edge
            shared_edges += 1;
        }
    }
    shared_edges
}

/// Breadth first search to fill around the shape with "water"
fn bfs(
    grid: &Vec<Vec<Vec<bool>>>,
    queue: &mut VecDeque<Point>,
    wet: &mut Vec<Vec<Vec<bool>>>,
) -> Vec<Vec<Vec<bool>>> {
    if queue.is_empty() {
        return wet.to_vec();
    }
    let start = queue.pop_front().unwrap();
    for next in get_open_neighbours(&start, grid) {
        if !wet[next.x][next.y][next.z] {
            wet[next.x][next.y][next.z] = true;
            queue.push_back(next);
        }
    }
    bfs(grid, queue, wet)
}

/// get adjacent points to p that are NOT occupieds
fn get_open_neighbours(p: &Point, grid: &Vec<Vec<Vec<bool>>>) -> Vec<Point> {
    let xlen = grid.len();
    let ylen = grid[0].len();
    let zlen = grid[0][0].len();
    let mut neighbours = Vec::<Point>::new();

    for xdelta in [-1, 1] {
        if ((p.x as i32) + xdelta < 0) | ((p.x as i32) + xdelta >= xlen as i32) {
            continue; // don't go outside of grid
        }
        if !grid[(p.x as i32 + xdelta) as usize][p.y][p.z] {
            //  free space
            neighbours.push(Point {
                x: (p.x as i32 + xdelta) as usize,
                y: p.y,
                z: p.z,
            });
        }
    }
    for ydelta in [-1, 1] {
        if ((p.y as i32) + ydelta < 0) | ((p.y as i32) + ydelta >= ylen as i32) {
            continue; // don't go outside of grid
        }
        if !grid[p.x][((p.y as i32) + ydelta) as usize][p.z] {
            //  free space
            neighbours.push(Point {
                x: p.x,
                y: (p.y as i32 + ydelta) as usize,
                z: p.z,
            });
        }
    }
    for zdelta in [-1, 1] {
        if ((p.z as i32) + zdelta < 0) | ((p.z as i32) + zdelta >= zlen as i32) {
            continue; // don't go outside of grid
        }
        if !grid[p.x][p.y][(p.z as i32 + zdelta) as usize] {
            //  free space
            neighbours.push(Point {
                x: p.x,
                y: p.y,
                z: (p.z as i32 + zdelta) as usize,
            });
        }
    }
    neighbours
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .map(|origin| Point {
            x: origin.0,
            y: origin.1,
            z: origin.2,
        })
        .collect_vec()
}

fn parse_grid(input: &str) -> (Vec<Vec<Vec<bool>>>, usize) {
    let border = 1; // make the grid slightly bigger than needed to allow BFS to search around the grid ("immerse the shape in water")
    let points = parse_points(input);
    let n_cubes = points.len();
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x + 2 * border;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y + 2 * border;
    let max_z = points.iter().max_by_key(|p| p.z).unwrap().z + 2 * border;

    let mut grid = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];

    for p in points {
        grid[p.x + border][p.y + border][p.z + border] = true;
    }
    (grid, n_cubes)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
