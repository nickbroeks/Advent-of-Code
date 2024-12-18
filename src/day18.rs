use std::{collections::VecDeque, usize};

use crate::day::Day;
pub struct Day18;

impl Day for Day18 {
    fn part1(&self, input: &str) -> String {
        let walls = parse_input(input);
        let size = 71;
        let bytes = 2935;
        let mut maze: Vec<Vec<bool>> = (0..size)
            .map(|_| (0..size).map(|_| false).collect())
            .collect();
        for i in 0..bytes {
            maze[walls[i].0][walls[i].1] = true;
        }
        solve_maze(maze, (0, 0), (size - 1, size - 1))
            .len()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let walls = parse_input(input);
        let size = 71;
        let mut i = 0;
        let mut j = walls.len();
        while j - i > 1 {
            let m = (j + i) / 2;
            let mut maze: Vec<Vec<bool>> = (0..size)
                .map(|_| (0..size).map(|_| false).collect())
                .collect();
            for i in 0..m {
                maze[walls[i].0][walls[i].1] = true;
            }
            let path = solve_maze(maze, (0, 0), (size - 1, size - 1));
            if path.len() == 0 {
                j = m;
            } else {
                i = m;
            }
        }
        return format!("{}, {}", walls[i].0.to_string(), walls[i].1.to_string());
    }
}

fn solve_maze(
    maze: Vec<Vec<bool>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut stack = VecDeque::new();
    stack.push_back((start, vec![]));
    let mut seen: Vec<Vec<bool>> = maze
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    while let Some((location, path)) = stack.pop_front() {
        if location == goal {
            return path;
        }
        if seen[location.0][location.1] {
            continue;
        }
        seen[location.0][location.1] = true;

        let mut new_path = path.clone();
        new_path.push(location);
        if location.0 > 0 && !maze[location.0 - 1][location.1] {
            stack.push_back(((location.0 - 1, location.1), new_path.clone()));
        }
        if location.0 < maze.len() - 1 && !maze[location.0 + 1][location.1] {
            stack.push_back(((location.0 + 1, location.1), new_path.clone()));
        }
        if location.1 > 0 && !maze[location.0][location.1 - 1] {
            stack.push_back(((location.0, location.1 - 1), new_path.clone()));
        }
        if location.1 < maze[0].len() - 1 && !maze[location.0][location.1 + 1] {
            stack.push_back(((location.0, location.1 + 1), new_path.clone()));
        }
    }
    vec![]
}
fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut split_line = line.split(",");
            (
                split_line.next().unwrap().parse().unwrap(),
                split_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}
