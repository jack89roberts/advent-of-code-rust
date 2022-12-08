fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut visible = vec![vec![false; ncols]; nrows];
    // look from left
    for i in 0..nrows {
        let mut max_height: i32 = -1;
        for j in 0..ncols {
            if grid[i][j] as i32 > max_height {
                max_height = grid[i][j] as i32;
                visible[i][j] = true;
            }
        }
    }
    // look from right
    for i in 0..nrows {
        let mut max_height: i32 = -1;
        for j in (0..ncols).rev() {
            if grid[i][j] as i32 > max_height {
                max_height = grid[i][j] as i32;
                visible[i][j] = true;
            }
        }
    }
    // look from top
    for j in 0..ncols {
        let mut max_height: i32 = -1;
        for i in 0..nrows {
            if grid[i][j] as i32 > max_height {
                max_height = grid[i][j] as i32;
                visible[i][j] = true;
            }
        }
    }
    // look from bottom
    for j in 0..ncols {
        let mut max_height: i32 = -1;
        for i in (0..nrows).rev() {
            if grid[i][j] as i32 > max_height {
                max_height = grid[i][j] as i32;
                visible[i][j] = true;
            }
        }
    }
    Some(
        visible
            .iter()
            .map(|row| row.iter().filter(|col| **col).count())
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut scenic_score = vec![vec![1; ncols]; nrows];

    for house_i in 0..nrows {
        for house_j in 0..ncols {
            // tree height at candidate house location
            let height = grid[house_i][house_j];
            // left
            let view_i: i32 = house_i as i32;
            let mut view_j: i32 = house_j as i32 - 1;
            let mut count: i32 = 0;
            while view_j >= 0 {
                // loop until left edge
                count += 1;
                if grid[view_i as usize][view_j as usize] >= height {
                    // stop when find a taller/same height tree
                    break;
                }
                view_j -= 1;
            }
            scenic_score[house_i][house_j] *= count;

            // right
            let view_i: i32 = house_i as i32;
            let mut view_j: i32 = house_j as i32 + 1;
            let mut count = 0;
            while view_j < ncols as i32 {
                // loop until right edge
                count += 1;
                if grid[view_i as usize][view_j as usize] >= height {
                    // stop when find a taller/same height tree
                    break;
                }
                view_j += 1;
            }
            scenic_score[house_i][house_j] *= count;

            // up
            let mut view_i: i32 = house_i as i32 - 1;
            let view_j: i32 = house_j as i32;
            let mut count = 0;
            while view_i >= 0 {
                // loop until top edge
                count += 1;
                if grid[view_i as usize][view_j as usize] >= height {
                    // stop when find a taller/same height tree
                    break;
                }
                view_i -= 1;
            }
            scenic_score[house_i][house_j] *= count;

            // down
            let mut view_i: i32 = house_i as i32 + 1;
            let view_j: i32 = house_j as i32;
            let mut count = 0;
            while view_i < nrows as i32 {
                // loop until bottom edge
                count += 1;
                if grid[view_i as usize][view_j as usize] >= height {
                    // stop when find a taller/same height tree
                    break;
                }
                view_i += 1;
            }
            scenic_score[house_i][house_j] *= count;
        }
    }

    Some(
        *scenic_score
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
