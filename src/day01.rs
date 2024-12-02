use std::collections::HashMap;

use crate::day::Day;
use crate::util::Pipe;

pub struct Day01;

impl Day for Day01 {
    fn part1(&self, input: &str) -> String {
        read_input(input)
            .pipe(|(mut first_list, mut second_list)| {
                first_list.sort_unstable();
                second_list.sort_unstable();
                first_list
                    .iter()
                    .zip(second_list)
                    .map(|(first, second)| first.abs_diff(second))
                    .sum::<u32>()
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        read_input(input)
            .pipe(|(first_list, second_list)| {
                let map_second_list =
                    second_list
                        .into_iter()
                        .fold(HashMap::new(), |mut map, num| {
                            *map.entry(num).or_insert(0) += 1;
                            map
                        });

                first_list
                    .iter()
                    .map(|num| num * map_second_list.get(num).unwrap_or(&0))
                    .sum::<u32>()
            })
            .to_string()
    }
}

fn read_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse().expect("Invalid number"));
            let first: u32 = nums.next().expect("Missing first number");
            let last: u32 = nums.next().expect("Missing last number");
            (first, last)
        })
        .unzip()
}
