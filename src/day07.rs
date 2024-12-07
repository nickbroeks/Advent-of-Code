use crate::{day::Day, util::Pipe};

pub struct Day07;

impl Day for Day07 {
    fn part1(&self, input: &str) -> String {
        parse_input(input)
            .iter()
            .filter(|&(sum, numbers)| {
                let mut numbers_iter = numbers.iter();
                let first_number = *numbers_iter.next().unwrap();

                numbers_iter
                    .fold(vec![first_number], |acc, &number| {
                        let mut temp = acc.iter().map(|&old| old + number).collect::<Vec<u64>>();

                        temp.append(&mut acc.iter().map(|&old| old * number).collect::<Vec<u64>>());
                        temp
                    })
                    .iter()
                    .any(|result| result == sum)
            })
            .map(|&(sum, _)| sum)
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse_input(input)
            .iter()
            .filter(|&(sum, numbers)| {
                let mut numbers_iter = numbers.iter();
                let first_number = *numbers_iter.next().unwrap();

                numbers_iter
                    .fold(vec![first_number], |acc, &number| {
                        let mut temp = acc.iter().map(|&old| old + number).collect::<Vec<u64>>();

                        temp.append(&mut acc.iter().map(|&old| old * number).collect::<Vec<u64>>());
                        temp.append(
                            &mut acc
                                .iter()
                                .map(|&old| {
                                    (old.to_string() + &number.to_string()).parse().unwrap()
                                })
                                .collect::<Vec<u64>>(),
                        );
                        temp.iter()
                            .filter(|&result| result <= sum)
                            .map(|&x| x)
                            .collect()
                    })
                    .iter()
                    .any(|result| result == sum)
            })
            .map(|&(sum, _)| sum)
            .sum::<u64>()
            .to_string()
    }
}
fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            line.split(": ").collect::<Vec<&str>>().pipe(|split| {
                (
                    split[0].parse().unwrap(),
                    split[1]
                        .split(" ")
                        .map(|num| num.parse().unwrap())
                        .collect(),
                )
            })
        })
        .collect()
}
