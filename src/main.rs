use std::env;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod util;

use day::Day;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = args[1]
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("Expected day number, got '{}'", &args[1]));
    let input_path = format!("inputs/day{:02}.txt", day);

    // Initialize available days
    let solutions: &[Box<dyn Day>] = &[
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
        Box::new(Day05),
        Box::new(Day06),
        Box::new(Day07),
        Box::new(Day08),
        Box::new(Day09),
        Box::new(Day10),
        Box::new(Day11),
    ];

    // Run the selected day's solution
    let solution = solutions
        .get(day - 1)
        .unwrap_or_else(|| panic!("Day {} is not implemented.", day));
    let input = std::fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));
    solution.run(&input);
}
