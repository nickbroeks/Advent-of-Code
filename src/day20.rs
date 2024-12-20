use std::{collections::VecDeque, usize};

use crate::day::Day;
pub struct Day20;

impl Day for Day20 {
    fn part1(&self, input: &str) -> String {
        let (start, end, maze) = parse_input(input);
        let distances_from_start = get_distances(start, &maze);
        let distances_from_end = get_distances(end, &maze);
        let best_time = distances_from_end[start.0][start.1];
        let mut count = 0;
        for i in 0..maze.len() as i32 {
            for j in 0..maze[0].len() as i32 {
                if maze[i as usize][j as usize] {
                    continue;
                }
                for a in -3..=3 {
                    for b in -3..=3 {
                        let cheat_time = (a as i32).abs() + (b as i32).abs();
                        if cheat_time > 2 {
                            continue;
                        }
                        if i + a >= 0
                            && i + a < maze.len() as i32
                            && j + b >= 0
                            && j + b < maze[0].len() as i32
                        {
                            if maze[(i + a) as usize][(j + b) as usize] {
                                continue;
                            }
                            if distances_from_start[i as usize][j as usize]
                                + distances_from_end[(i + a) as usize][(j + b) as usize]
                                + (cheat_time) as usize
                                <= best_time - 100
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (start, end, maze) = parse_input(input);
        let distances_from_start = get_distances(start, &maze);
        let distances_from_end = get_distances(end, &maze);
        let best_time = distances_from_end[start.0][start.1];
        let mut count = 0;
        for i in 0..maze.len() as i32 {
            for j in 0..maze[0].len() as i32 {
                if maze[i as usize][j as usize] {
                    continue;
                }
                for a in -20..=20 {
                    for b in -20..=20 {
                        let cheat_time = (a as i32).abs() + (b as i32).abs();
                        if cheat_time > 20 {
                            continue;
                        }
                        if i + a >= 0
                            && i + a < maze.len() as i32
                            && j + b >= 0
                            && j + b < maze[0].len() as i32
                        {
                            if maze[(i + a) as usize][(j + b) as usize] {
                                continue;
                            }
                            if distances_from_start[i as usize][j as usize]
                                + distances_from_end[(i + a) as usize][(j + b) as usize]
                                + (cheat_time) as usize
                                <= best_time - 100
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
        count.to_string()
    }
}

fn get_distances(position: (usize, usize), maze: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let mut distances: Vec<Vec<usize>> = maze
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect())
        .collect();
    let mut queue = VecDeque::new();
    queue.push_back((position, 0));
    while let Some((pos, dist)) = queue.pop_front() {
        if distances[pos.0][pos.1] != usize::MAX {
            continue;
        }
        distances[pos.0][pos.1] = dist;
        if pos.0 > 0 && !maze[pos.0 - 1][pos.1] {
            queue.push_back(((pos.0 - 1, pos.1), dist + 1));
        }
        if pos.0 < distances.len() - 1 && !maze[pos.0 + 1][pos.1] {
            queue.push_back(((pos.0 + 1, pos.1), dist + 1));
        }
        if pos.1 > 0 && !maze[pos.0][pos.1 - 1] {
            queue.push_back(((pos.0, pos.1 - 1), dist + 1));
        }
        if pos.1 < distances[0].len() - 1 && !maze[pos.0][pos.1 + 1] {
            queue.push_back(((pos.0, pos.1 + 1), dist + 1));
        }
    }

    distances
}

fn parse_input(input: &str) -> ((usize, usize), (usize, usize), Vec<Vec<bool>>) {
    let lines = input.lines();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = Vec::new();
    for (i, line) in lines.enumerate() {
        walls.push(vec![]);
        for (j, char) in line.chars().enumerate() {
            walls[i].push(char == '#');
            if char == 'S' {
                start = (i, j);
            }
            if char == 'E' {
                end = (i, j);
            }
        }
    }
    (start, end, walls)
}
