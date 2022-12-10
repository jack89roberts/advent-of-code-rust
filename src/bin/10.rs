use itertools::Itertools;

fn update(
    cycle: &mut i32,
    x: &mut i32,
    signal: &mut i32,
    display: &mut String,
    store_cycles: &[i32],
) {
    *cycle += 1;
    if store_cycles.contains(cycle) {
        *signal += (*x) * (*cycle);
    }
    let pixel: i32 = (*cycle - 1) % 40;
    if (pixel - *x).abs() <= 1 {
        *display += "#";
    } else {
        *display += ".";
    }
    if *cycle % 40 == 0 {
        *display += "\n";
    }
}

pub fn run(input: &str) -> (i32, String) {
    let mut cycle = 0;
    let mut x = 1;
    let store_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut signal = 0;
    let mut display: String = "".to_string();
    for instr in input.lines() {
        update(&mut cycle, &mut x, &mut signal, &mut display, &store_cycles);
        match instr {
            "noop" => {
                continue;
            }
            _ => {
                // addx
                update(&mut cycle, &mut x, &mut signal, &mut display, &store_cycles);
                x += instr.split(' ').collect_vec()[1].parse::<i32>().unwrap();
            }
        }
    }
    (signal, display)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (signal, _) = run(input);
    Some(signal as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, display) = run(input);
    Some(display)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n"
                .to_string()
            )
        );
    }
}
