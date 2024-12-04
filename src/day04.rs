use crate::{day::Day, util::Pipe};

pub struct Day04;

impl Day for Day04 {
    fn part1(&self, input: &str) -> String {
        parse_input(input)
            .pipe(|matrix| {
                let mut sum = 0;
                for i in 0..matrix.len() {
                    for j in 0..matrix[i].len() - 3 {
                        let word: Vec<char> = matrix[i][j..j + 4].to_vec();
                        if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                            sum += 1;
                        }
                    }
                }
                for i in 0..matrix.len() - 3 {
                    for j in 0..matrix[i].len() {
                        let word: Vec<char> = matrix[i..i + 4].iter().map(|row| row[j]).collect();
                        if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                            sum += 1;
                        }
                    }
                }
                for i in 0..matrix.len() - 3 {
                    for j in 0..matrix[i].len() - 3 {
                        let word = matrix[i..i + 4]
                            .iter()
                            .enumerate()
                            .map(|(k, row)| row[j + k])
                            .collect::<Vec<char>>();
                        if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                            sum += 1;
                        }
                        let word = matrix[i..i + 4]
                            .iter()
                            .enumerate()
                            .map(|(k, row)| row[j + 3 - k])
                            .collect::<Vec<char>>();
                        if word == ['X', 'M', 'A', 'S'] || word == ['S', 'A', 'M', 'X'] {
                            sum += 1;
                        }
                    }
                }
                sum
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        parse_input(input)
            .pipe(|matrix| {
                let mut sum = 0;
                for i in 0..matrix.len() - 2 {
                    for j in 0..matrix[i].len() - 2 {
                        let square: Vec<Vec<char>> = matrix[i..i + 3]
                            .iter()
                            .map(|row| row.get(j..j + 3).unwrap().to_vec())
                            .collect();
                        let word1 = square
                            .iter()
                            .enumerate()
                            .map(|(k, row)| row[k])
                            .collect::<Vec<char>>();
                        let word2 = square
                            .iter()
                            .enumerate()
                            .map(|(k, row)| row[2 - k])
                            .collect::<Vec<char>>();
                        if (word1 == ['M', 'A', 'S'] || word1 == ['S', 'A', 'M'])
                            && (word2 == ['M', 'A', 'S'] || word2 == ['S', 'A', 'M'])
                        {
                            sum += 1;
                        }
                    }
                }
                sum
            })
            .to_string()
    }
}
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
