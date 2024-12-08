use std::collections::{HashMap, HashSet};

use crate::day::Day;

pub struct Day08;

impl Day for Day08 {
    fn part1(&self, input: &str) -> String {
        let mut result = HashSet::new();
        parse_input(input)
            .values()
            .flat_map(|indices| {
                indices.iter().flat_map(|a| {
                    indices
                        .iter()
                        .filter(move |&b| a.0 != b.0 && a.1 != b.1)
                        .map(|b| (a.0 + a.0 - b.0, a.1 + a.1 - b.1))
                })
            })
            .for_each(|coord| {
                if coord.0 >= 0
                    && coord.0 < input.lines().count() as i32
                    && coord.1 >= 0
                    && coord.1 < input.lines().next().unwrap().len() as i32
                {
                    result.insert(coord);
                }
                ()
            });
        result.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut result = HashSet::new();
        parse_input(input)
            .values()
            .flat_map(|indices| {
                indices.iter().flat_map(|a| {
                    indices
                        .iter()
                        .filter(move |&b| a.0 != b.0 && a.1 != b.1)
                        .flat_map(|b| {
                            (0..10000000)
                                .map(|i| (a.0 + i * (a.0 - b.0), a.1 + i * (a.1 - b.1)))
                                .take_while(|&(i, j)| {
                                    i >= 0
                                        && i < input.lines().count() as i32
                                        && j >= 0
                                        && j < input.lines().next().unwrap().len() as i32
                                })
                        })
                })
            })
            .for_each(|coord| {
                result.insert(coord);
                ()
            });
        result.len().to_string()
    }
}
fn parse_input(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut indices = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, char)| {
            if char != '.' {
                indices
                    .entry(char)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        })
    });
    indices
}
