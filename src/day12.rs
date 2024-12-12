use crate::day::Day;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day12;

impl Day for Day12 {
    fn part1(&self, input: &str) -> String {
        let input = parse_input(input);
        let regions = get_regions(&input);
        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        for i in 0..regions.len() {
            for j in 0..regions[i].len() {
                let (area, perimiter) = map.entry(regions[i][j]).or_insert((0, 0));
                *area += 1;
                *perimiter += [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .filter(|&&(di, dj)| {
                        let newi = i as i32 + di;
                        let newj = j as i32 + dj;
                        newi < 0
                            || newi >= regions.len() as i32
                            || newj < 0
                            || newj >= regions[i].len() as i32
                            || regions[i][j] != regions[newi as usize][newj as usize]
                    })
                    .count();
            }
        }
        map.values()
            .map(|&(area, perimiter)| area * perimiter)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let input = parse_input(input);
        let mut regions = get_regions(&input);
        let regions = pad(&mut regions);
        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        for i in 1..regions.len() - 1 {
            for j in 1..regions[i].len() - 1 {
                let label = regions[i][j];
                let (area, sides) = map.entry(label).or_insert((0, 0));
                *area += 1;
                if regions[i - 1][j] != label
                    && ((regions[i][j - 1] != label) || (regions[i - 1][j - 1] == label))
                {
                    *sides += 1;
                }
                if regions[i][j - 1] != label
                    && ((regions[i - 1][j] != label) || (regions[i - 1][j - 1] == label))
                {
                    *sides += 1;
                }
                if regions[i + 1][j] != label
                    && ((regions[i][j - 1] != label) || (regions[i + 1][j - 1] == label))
                {
                    *sides += 1;
                }
                if regions[i][j + 1] != label
                    && ((regions[i - 1][j] != label) || (regions[i - 1][j + 1] == label))
                {
                    *sides += 1;
                }
            }
        }
        map.values()
            .map(|&(area, sides)| area.to_string() + " " + &sides.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .to_string();
        map.values()
            .map(|&(area, perimiter)| area * perimiter)
            .sum::<usize>()
            .to_string()
    }
}

fn get_regions(input: &Vec<Vec<u32>>) -> Vec<Vec<i32>> {
    let mut regions: Vec<Vec<i32>> = (0..input.len())
        .map(|row| (0..input[row].len()).map(|_| -1).collect())
        .collect();
    let mut region = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if regions[i][j] != -1 {
                continue;
            }
            flood_fill(input, &mut regions, (i, j), region);
            region += 1;
        }
    }
    regions
}
fn flood_fill(
    input: &Vec<Vec<u32>>,
    regions: &mut Vec<Vec<i32>>,
    node: (usize, usize),
    region: i32,
) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    q.push_back(node);
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        if input[n.0][n.1] != input[node.0][node.1] {
            continue;
        }
        regions[n.0][n.1] = region;
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let newi = n.0 as i32 + di;
            let newj = n.1 as i32 + dj;
            if newi < 0
                || newi >= regions.len() as i32
                || newj < 0
                || newj >= regions[n.0].len() as i32
            {
                continue;
            }
            let newi = newi as usize;
            let newj = newj as usize;
            if seen.contains(&(newi, newj)) {
                continue;
            }
            seen.insert((newi, newj));
            if regions[newi][newj] == -1 && input[newi][newj] == input[node.0][node.1] {
                q.push_back((newi, newj));
            }
        }
    }
}
fn pad(regions: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    regions.insert(0, (0..regions[0].len()).map(|_| -1).collect::<Vec<i32>>());
    regions.push((0..regions[0].len()).map(|_| -1).collect::<Vec<i32>>());
    regions
        .iter()
        .map(|row| {
            vec![-1]
                .iter()
                .chain(row)
                .chain(vec![-1].iter())
                .map(|&x| x)
                .collect::<Vec<i32>>()
        })
        .collect()
}
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c as u32).collect())
        .collect()
}
