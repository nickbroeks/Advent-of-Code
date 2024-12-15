use std::collections::VecDeque;

use crate::day::Day;
pub struct Day15;

impl Day for Day15 {
    fn part1(&self, input: &str) -> String {
        let (walls, mut boxes, mut location, moves) = parse_input(input);
        for m in moves {
            let new_location = (
                (location.0 as i32 + m.0) as usize,
                (location.1 as i32 + m.1) as usize,
            );
            let mut check_location = new_location;
            while !walls[check_location.0][check_location.1]
                && boxes[check_location.0][check_location.1]
            {
                check_location = (
                    (check_location.0 as i32 + m.0) as usize,
                    (check_location.1 as i32 + m.1) as usize,
                );
            }
            if walls[check_location.0][check_location.1] {
                continue;
            }
            location = new_location;
            boxes[check_location.0][check_location.1] = true;
            boxes[new_location.0][new_location.1] = false;
        }
        boxes
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, &is_box)| if is_box { i * 100 + j } else { 0 })
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (walls, mut boxes, mut location, moves) = parse_input_2(input);
        for m in moves {
            if m.0 == 0 {
                let new_j = (location.1 as i32 + m.1) as usize;
                let mut old_j = location.1 as i32;
                while !walls[location.0][(old_j + m.1) as usize]
                    && boxes[location.0][(old_j + (if m.1 == 1 { 1 } else { -2 })) as usize]
                {
                    old_j = old_j + 2 * m.1;
                }
                if walls[location.0][(old_j + m.1) as usize] {
                    continue;
                }
                while old_j != location.1 as i32 {
                    old_j = old_j - 2 * m.1;
                    boxes[location.0][(old_j + (if m.1 == 1 { 1 } else { -2 })) as usize] = false;
                    boxes[location.0][((old_j + (if m.1 == 1 { 1 } else { -2 })) + m.1) as usize] =
                        true;
                }
                location = (location.0, new_j);
            } else {
                let new_location = ((location.0 as i32 + m.0) as usize, location.1);
                let mut front = VecDeque::new();
                if walls[new_location.0][new_location.1] {
                    continue;
                }
                front.push_back(new_location);
                front.push_back((new_location.0, new_location.1 - 1));
                let mut failed = false;
                let mut stack = Vec::new();
                while !front.is_empty() {
                    let check_location = front.pop_front().unwrap();
                    if !boxes[check_location.0][check_location.1] {
                        continue;
                    }
                    if walls[(check_location.0 as i32 + m.0) as usize][check_location.1]
                        || walls[(check_location.0 as i32 + m.0) as usize][check_location.1 + 1]
                    {
                        failed = true;
                        break;
                    }
                    stack.push(check_location);
                    if !front.contains(&(
                        (check_location.0 as i32 + m.0) as usize,
                        check_location.1 - 1,
                    )) {
                        front.push_back((
                            (check_location.0 as i32 + m.0) as usize,
                            check_location.1 - 1,
                        ));
                    }
                    if !front
                        .contains(&((check_location.0 as i32 + m.0) as usize, check_location.1))
                    {
                        front.push_back((
                            (check_location.0 as i32 + m.0) as usize,
                            check_location.1,
                        ));
                    }
                    if !front.contains(&(
                        (check_location.0 as i32 + m.0) as usize,
                        check_location.1 + 1,
                    )) {
                        front.push_back((
                            (check_location.0 as i32 + m.0) as usize,
                            check_location.1 + 1,
                        ));
                    }
                }
                if failed {
                    continue;
                }
                location = new_location;
                while stack.len() > 0 {
                    let loc = stack.pop().unwrap();
                    boxes[loc.0][loc.1] = false;
                    boxes[(loc.0 as i32 + m.0) as usize][loc.1] = true;
                }
            }
            // println!(
            //     "{}",
            //     boxes
            //         .iter()
            //         .enumerate()
            //         .map(|(i, row)| {
            //             row.iter()
            //                 .enumerate()
            //                 .map(|(j, &is_box)| {
            //                     if is_box {
            //                         "["
            //                     } else if j != 0 && boxes[i][j - 1] {
            //                         "]"
            //                     } else if walls[i][j] {
            //                         "#"
            //                     } else {
            //                         " "
            //                     }
            //                 })
            //                 .collect::<Vec<_>>()
            //                 .join("")
            //         })
            //         .collect::<Vec<_>>()
            //         .join("\n")
            // );
        }
        boxes
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, &is_box)| if is_box { i * 100 + j } else { 0 })
            })
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(
    input: &str,
) -> (
    Vec<Vec<bool>>,
    Vec<Vec<bool>>,
    (usize, usize),
    Vec<(i32, i32)>,
) {
    let mut walls = Vec::new();
    let mut boxes = Vec::new();
    let mut location = (0, 0);
    let mut lines = input.lines();
    let mut i = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        walls.push(Vec::new());
        boxes.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                location = (i, j)
            }
            walls[i].push(c == '#');
            boxes[i].push(c == 'O');
        }
        i += 1;
    }
    let moves: Vec<_> = lines
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => (0, -1),
                'v' => (1, 0),
                '>' => (0, 1),
                '^' => (-1, 0),
                _ => panic!("WHAT????"),
            })
        })
        .collect();
    (walls, boxes, location, moves)
}

fn parse_input_2(
    input: &str,
) -> (
    Vec<Vec<bool>>,
    Vec<Vec<bool>>,
    (usize, usize),
    Vec<(i32, i32)>,
) {
    let mut walls = Vec::new();
    let mut boxes = Vec::new();
    let mut location = (0, 0);
    let mut lines = input.lines();
    let mut i = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        walls.push(Vec::new());
        boxes.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            if c == '@' {
                location = (i, 2 * j)
            }
            walls[i].push(c == '#');
            walls[i].push(c == '#');
            boxes[i].push(c == 'O');
            boxes[i].push(false);
        }
        i += 1;
    }
    let moves: Vec<_> = lines
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => (0, -1),
                'v' => (1, 0),
                '>' => (0, 1),
                '^' => (-1, 0),
                _ => panic!("WHAT????"),
            })
        })
        .collect();
    (walls, boxes, location, moves)
}
