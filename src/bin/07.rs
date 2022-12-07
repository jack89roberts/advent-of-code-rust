use std::{collections::HashMap, path::PathBuf};

struct File {
    size: u32,
}
struct Dir {
    children: Vec<PathBuf>,
    files: Vec<File>,
    size: u32,
}

fn parse_input(input: &str) -> HashMap<PathBuf, Dir> {
    let mut tree = HashMap::new();

    // route to current directory: last element is current directory,
    // elements before are directories traversed to get there
    let mut cd: Vec<PathBuf> = Vec::new();

    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "$" => {
                // this is a command
                match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            // move up to parent directory
                            cd.pop();
                        }
                        _ => {
                            // create and move into child directory
                            // cd[cd.len() - 1] is the current directory
                            // parts[2] is the relative name of the new directory
                            if cd.is_empty() {
                                cd.push(PathBuf::from(parts[2])); // if this is thhe first dir no previous path to join
                            } else {
                                cd.push(cd[cd.len() - 1].join(parts[2]));
                            }

                            tree.insert(
                                cd[cd.len() - 1].to_path_buf(),
                                Dir {
                                    children: Vec::<PathBuf>::new(),
                                    files: Vec::<File>::new(),
                                    size: 0,
                                },
                            );
                        }
                    },
                    _ => continue, // ls: nothing to do, will parse listed files in next iterations
                };
            }
            "dir" => {
                // add listed directory to current directory's children
                // cd[cd.len() - 1] is the current directory
                tree.get_mut(&cd[cd.len() - 1])
                    .unwrap()
                    .children
                    .push(cd[cd.len() - 1].join(parts[1]));
            }
            _ => {
                // this is a file size + file name
                let f = File {
                    size: parts[0].parse::<u32>().unwrap(),
                };
                // add the file's size to all the directory sizes in the current hierarchy
                for dir in &cd {
                    tree.get_mut(dir).unwrap().size += f.size;
                }
                // add the file to the current directory's files
                tree.get_mut(&cd[cd.len() - 1]).unwrap().files.push(f);
            }
        }
    }
    tree
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse_input(input);
    Some(tree.values().fold(0, |sum, dir| {
        if dir.size <= 100000 {
            sum + dir.size
        } else {
            sum
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = parse_input(input);
    let space_needed = 30000000 - (70000000 - tree.get(&PathBuf::from("/")).unwrap().size);
    Some(tree.values().fold(u32::MAX, |size, dir| {
        if (dir.size >= space_needed) & (dir.size < size) {
            dir.size
        } else {
            size
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
