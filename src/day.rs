pub trait Day {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;

    fn run(&self, input: &str) {
        println!("Part 1: {}", self.part1(input));
        println!("Part 2: {}", self.part2(input));
    }
}