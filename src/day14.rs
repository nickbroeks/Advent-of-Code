use crate::day::Day;
use regex::Regex;
pub struct Day14;

impl Day for Day14 {
    fn part1(&self, input: &str) -> String {
        let w = 101;
        let h = 103;
        let hw = 50;
        let hh = 51;
        parse_input(input)
            .iter()
            .map(|&(pos, vel)| {
                (
                    (pos.0 + vel.0 * 100).rem_euclid(w),
                    (pos.1 + vel.1 * 100).rem_euclid(h),
                )
            })
            .fold(vec![0, 0, 0, 0], |mut acc, (x, y)| {
                if x != hw && y != hh {
                    acc[((i64::signum(hw - x) + 1) / 2 + (i64::signum(hh - y) + 1)) as usize] += 1;
                }
                acc
            })
            .iter()
            .fold(1, |a, b| a * b)
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let w = 101;
        let h = 103;

        let input = parse_input(input);
        for i in 0..10000 {
            let mut canvas: Vec<Vec<char>> = (0..h)
                .map(|y| (0..w).map(|_| ' ').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            for &(pos, vel) in input.iter() {
                canvas[(pos.1 + vel.1 * i).rem_euclid(h) as usize]
                    [(pos.0 + vel.0 * i).rem_euclid(w) as usize] = '.';
            }
            println!("{}", i);
            println!(
                "{}",
                canvas
                    .iter()
                    .map(|line| line
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(""))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
        "".to_string()
    }
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input
        .lines()
        .map(|line| {
            let cap = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
                .unwrap()
                .captures(line)
                .unwrap();
            (
                (
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}
