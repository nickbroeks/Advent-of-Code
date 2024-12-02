use crate::day::Day;
use crate::util::Pipe;

pub struct Day02;

impl Day for Day02 {
    fn part1(&self, input: &str) -> String {
        read_input(input)
            .iter()
            .map(|line| {
                line.windows(2)
                    .map(|window| window[0] - window[1])
                    .collect::<Vec<_>>()
                    .pipe(|differences| {
                        [1, -1].iter().any(|&direction| {
                            differences
                                .iter()
                                .all(|&diff| (1..=3).contains(&(diff * direction)))
                        })
                    })
            })
            .filter(|&x| x)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        read_input(input)
            .iter()
            .map(|line| {
                line.windows(2)
                    .map(|window| window[0] - window[1])
                    .collect::<Vec<_>>()
                    .pipe(|differences| can_form_sequence_with_skip(&differences))
            })
            .filter(|&x| x)
            .count()
            .to_string()
    }
}

fn can_form_sequence_with_skip(differences: &[i32]) -> bool {
    [1, -1]
        .iter()
        .any(|&direction| can_form_directed_sequence_with_skip(&differences, direction))
}
fn can_form_directed_sequence_with_skip(differences: &[i32], direction: i32) -> bool {
    let n = differences.len();
    let mut has_skipped = false;
    let mut i = 0;
    while i < n {
        let diff = differences[i] * direction;
        if (1..=3).contains(&(diff)) {
            i += 1;
            continue;
        }
        if has_skipped {
            return false;
        }
        has_skipped = true;
        if i == n - 1 {
            i += 1;
            continue;
        }
        if (1..=3).contains(&(differences[i + 1] * direction + diff)) {
            i += 2;
            continue;
        }
        if i == 0 {
            i += 1;
            continue;
        }
        if (1..=3).contains(&(differences[i - 1] * direction + diff)) {
            i += 1;
            continue;
        }
        return false;
    }
    true
}

fn read_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().expect("Invalid number"))
                .collect()
        })
        .collect()
}
