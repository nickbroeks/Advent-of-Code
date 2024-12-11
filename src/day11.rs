use std::collections::HashMap;

use crate::day::Day;

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: &str) -> String {
        exec(input, 25)
    }

    fn part2(&self, input: &str) -> String {
        exec(input, 75)
    }
}

fn exec(input: &str, count: usize) -> String {
    let stones = parse_input(input);
    let mut stone_map: HashMap<i64, i64> = HashMap::new();
    for stone in stones {
        stone_map.insert(
            stone.parse().unwrap(),
            stone_map.get(&stone.parse().unwrap()).unwrap_or_else(|| &0) + 1,
        );
    }
    for _ in 0..count {
        let mut tempstones = HashMap::new();
        for &stone in stone_map.keys() {
            if stone == 0 {
                tempstones.insert(
                    1,
                    tempstones.get(&1).unwrap_or_else(|| &0) + stone_map.get(&stone).unwrap(),
                );
            } else if stone.to_string().len() % 2 == 0 {
                let mut stone_string = stone.to_string();
                let second_half = stone_string.split_off(stone_string.len() / 2);
                let second_half = second_half.trim_start_matches("0");
                let second_half = if second_half == "" { "0" } else { second_half };

                tempstones.insert(
                    stone_string.parse().unwrap(),
                    tempstones
                        .get(&stone_string.parse().unwrap())
                        .unwrap_or_else(|| &0)
                        + stone_map.get(&stone).unwrap(),
                );

                tempstones.insert(
                    second_half.parse().unwrap(),
                    tempstones
                        .get(&second_half.parse().unwrap())
                        .unwrap_or_else(|| &0)
                        + stone_map.get(&stone).unwrap(),
                );
            } else {
                tempstones.insert(
                    stone * 2024,
                    tempstones.get(&(stone * 2024)).unwrap_or_else(|| &0)
                        + stone_map.get(&stone).unwrap(),
                );
            }
        }
        stone_map = tempstones;
    }
    stone_map.values().sum::<i64>().to_string()
}
fn parse_input(input: &str) -> Vec<String> {
    input.split(" ").map(|s| s.to_string()).collect()
}
