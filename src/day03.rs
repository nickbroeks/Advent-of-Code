use crate::day::Day;
use regex::Regex;

pub struct Day03;

impl Day for Day03 {
    fn part1(&self, input: &str) -> String {
        let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        input
            .lines()
            .flat_map(|line| {
                r.captures_iter(line)
                    .map(|c| c.extract())
                    .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let r = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))()|(don't\(\))()").unwrap();
        r.captures_iter(input)
            .map(|c| c.extract())
            .fold((true, 0), |mut acc, (word, [a, b])| {
                match word {
                    "do()" => {
                        acc.0 = true;
                    }
                    "don't()" => {
                        acc.0 = false;
                    }
                    _ => {
                        if acc.0 {
                            acc.1 += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                        }
                    }
                }
                acc
            })
            .1
            .to_string()
    }
}
