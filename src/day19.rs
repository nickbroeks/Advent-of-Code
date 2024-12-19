use std::collections::HashMap;

use crate::day::Day;
pub struct Day19;

impl Day for Day19 {
    fn part1(&self, input: &str) -> String {
        let (available_patterns, goals) = parse_input(input);
        goals
            .iter()
            .filter(|&&goal| try_pattern(&available_patterns, goal))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (available_patterns, goals) = parse_input(input);
        let mut cache = HashMap::new();
        goals
            .iter()
            .map(|&goal| count_patterns(&available_patterns, goal, &mut cache))
            .sum::<i64>()
            .to_string()
    }
}
fn try_pattern(available_patterns: &Vec<&str>, goal: &str) -> bool {
    if goal.len() == 0 {
        return true;
    }
    available_patterns
        .iter()
        .any(|&pat| goal.starts_with(pat) && try_pattern(available_patterns, &goal[pat.len()..]))
}

fn count_patterns(
    available_patterns: &Vec<&str>,
    goal: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if goal.len() == 0 {
        return 1;
    }
    let goal_string = goal.to_string();
    if cache.contains_key(&goal_string) {
        return *cache.get(&goal_string).unwrap();
    }

    let score = available_patterns
        .iter()
        .filter(|&&pat| goal.starts_with(pat))
        .map(|&pat| count_patterns(available_patterns, &goal[pat.len()..], cache))
        .sum();
    cache.insert(goal_string, score);
    score
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let available_patterns = lines.next().unwrap().split(", ").collect();
    lines.next();
    let goals = lines.collect();
    (available_patterns, goals)
}
