use std::collections::HashMap;

use crate::day::Day;

pub struct Day06;

impl Day for Day06 {
    fn part1(&self, input: &str) -> String {
        let (maze, mut location, mut direction) = parse_input(input);
        let mut seen: Vec<Vec<bool>> = maze
            .iter()
            .map(|row| row.iter().map(|_cell| false).collect())
            .collect();
        while location.0 >= 0
            && location.0 < maze.len() as i32
            && location.1 >= 0
            && location.1 < maze[0].len() as i32
        {
            seen[location.0 as usize][location.1 as usize] = true;
            let mut next = (location.0 + direction.0, location.1 + direction.1);
            for _ in 0..4 {
                if next.0 >= 0
                    && next.0 < maze.len() as i32
                    && next.1 >= 0
                    && next.1 < maze[0].len() as i32
                    && maze[next.0 as usize][next.1 as usize]
                {
                    direction = (direction.1, -direction.0);
                    next = (location.0 + direction.0, location.1 + direction.1);
                } else {
                    break;
                }
            }
            location = next;
        }
        seen.iter()
            .map(|row| row.iter().filter(|&&cell| cell).count())
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut dirs = HashMap::new();
        dirs.insert((0, 1), 0);
        dirs.insert((1, 0), 1);
        dirs.insert((0, -1), 2);
        dirs.insert((-1, 0), 3);
        let (maze, start, start_direction) = parse_input(input);
        (0..maze.len())
            .map(|y| {
                (0..maze[y].len())
                    .filter(|&x| {
                        let mut location = start;
                        let mut direction = start_direction;
                        let mut seen: Vec<Vec<[bool; 4]>> = maze
                            .iter()
                            .map(|row| {
                                row.iter()
                                    .map(|_cell| [false, false, false, false])
                                    .collect()
                            })
                            .collect();
                        while location.0 >= 0
                            && location.0 < maze.len() as i32
                            && location.1 >= 0
                            && location.1 < maze[0].len() as i32
                        {
                            let dir_index = *dirs.get(&direction).unwrap();
                            if seen[location.0 as usize][location.1 as usize][dir_index] {
                                return true;
                            }
                            seen[location.0 as usize][location.1 as usize][dir_index] = true;
                            let mut next = (location.0 + direction.0, location.1 + direction.1);
                            for _ in 0..4 {
                                let temp = next.0 as usize;
                                let is_new_wall = temp == y && next.1 as usize == x;
                                if next.0 >= 0
                                    && next.0 < maze.len() as i32
                                    && next.1 >= 0
                                    && next.1 < maze[0].len() as i32
                                    && (is_new_wall || maze[next.0 as usize][next.1 as usize])
                                {
                                    direction = (direction.1, -direction.0);
                                    next = (location.0 + direction.0, location.1 + direction.1);
                                } else {
                                    break;
                                }
                            }
                            location = next;
                        }
                        false
                    })
                    .count() as u32
            })
            .sum::<u32>()
            .to_string()
    }
}
fn parse_input(input: &str) -> (Vec<Vec<bool>>, (i32, i32), (i32, i32)) {
    let maze = input.lines().collect::<Vec<&str>>();
    let position = maze
        .iter()
        .position(|&line| line.chars().any(|c| !['.', '#'].contains(&c)))
        .unwrap();
    let position = (
        position as i32,
        maze[position]
            .chars()
            .position(|c| !['.', '#'].contains(&c))
            .unwrap() as i32,
    );
    (
        maze.iter()
            .map(|&line| line.chars().map(|c| c == '#').collect())
            .collect(),
        position,
        (-1, 0),
    )
}
