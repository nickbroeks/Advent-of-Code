use std::env;

mod day;
mod day01;

use day::Day;

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
    let solutions: [Box<dyn Day>; 0] = [];

    // Run the selected day's solution
    let solution = solutions
        .get(day - 1)
        .unwrap_or_else(|| panic!("Day {} is not implemented.", day));
    let input = std::fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", input_path));
    solution.run(&input);
}
