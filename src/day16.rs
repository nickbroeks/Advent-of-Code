use std::collections::{HashMap, HashSet};

use crate::day::Day;
pub struct Day16;

impl Day for Day16 {
    fn part1(&self, input: &str) -> String {
        let (walls, start, end) = parse_input(input);
        let mut seen = HashSet::new();
        let mut queue = Vec::new();
        let first_state = (start, (0, 1), 0);
        queue.push(first_state);
        while queue.len() > 0 {
            let (location, direction, cost) = queue.pop().unwrap();
            if seen.contains(&(location, direction)) {
                continue;
            }
            seen.insert((location, direction));
            if location == end {
                queue.clear();
                queue.push((location, direction, cost));
                break;
            }
            let new_location = (location.0 + direction.0, location.1 + direction.1);
            if new_location.0 >= 0
                && new_location.0 < walls.len() as i32
                && new_location.1 >= 0
                && new_location.1 < walls[0].len() as i32
                && !walls[new_location.0 as usize][new_location.1 as usize]
            {
                queue.push((new_location, direction, cost + 1));
            }
            queue.push((location, (direction.1, direction.0), cost + 1000));
            queue.push((location, (-direction.1, -direction.0), cost + 1000));
            queue.sort_by_key(|&(_, _, cost)| -cost);
        }
        let (_, _, cost) = queue.pop().unwrap();
        cost.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (walls, start, end) = parse_input(input);
        let mut seen = HashMap::new();
        let mut queue = Vec::new();
        let first_state = (start, (0, 1), 0, vec![]);
        queue.push(first_state);
        while queue.len() > 0 {
            let (location, direction, cost, route) = queue.pop().unwrap();
            if location == end {
                queue.push((location, direction, cost, route));
                queue = queue
                    .iter()
                    .filter(|&&(location2, _, cost2, _)| location == location2 && cost == cost2)
                    .map(|(location, dir, cost, nodes)| (*location, *dir, *cost, nodes.clone()))
                    .collect();
                break;
            }
            let new_location = (location.0 + direction.0, location.1 + direction.1);
            if new_location.0 >= 0
                && new_location.0 < walls.len() as i32
                && new_location.1 >= 0
                && new_location.1 < walls[0].len() as i32
                && !walls[new_location.0 as usize][new_location.1 as usize]
                && (!seen.contains_key(&(new_location, direction))
                    || seen[&(new_location, direction)] >= cost + 1)
            {
                let mut route = route.clone();
                route.push(location);
                queue.push((new_location, direction, cost + 1, route));

                seen.insert((new_location, direction), cost + 1);
            }
            if !seen.contains_key(&(location, (direction.1, direction.0)))
                || seen[&(location, (direction.1, direction.0))] >= cost + 1000
            {
                queue.push((
                    location,
                    (direction.1, direction.0),
                    cost + 1000,
                    route.clone(),
                ));

                seen.insert((location, (direction.1, direction.0)), cost + 1000);
            }
            if !seen.contains_key(&(location, (-direction.1, -direction.0)))
                || seen[&(location, (-direction.1, -direction.0))] >= cost + 1000
            {
                queue.push((
                    location,
                    (-direction.1, -direction.0),
                    cost + 1000,
                    route.clone(),
                ));

                seen.insert((location, (-direction.1, -direction.0)), cost + 1000);
            }
            queue.sort_by_key(|&(_, _, cost, _)| -cost);
        }
        let mut seen = HashSet::new();
        for (location, _, _, route) in queue {
            for node in route {
                seen.insert(node);
            }
            seen.insert(location);
        }
        seen.len().to_string()
    }
}

fn parse_input(input: &str) -> (Vec<Vec<bool>>, (i32, i32), (i32, i32)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (j, char) in line.chars().enumerate() {
            row.push(char == '#');
            if char == 'S' {
                start = (i as i32, j as i32);
            } else if char == 'E' {
                end = (i as i32, j as i32);
            }
        }
        walls.push(row);
    }
    (walls, start, end)
}
