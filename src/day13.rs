use crate::day::Day;

pub struct Day13;

impl Day for Day13 {
    fn part1(&self, input: &str) -> String {
        let input = parse_input(input);
        input
            .iter()
            .map(|&((ax, ay), (bx, by), (x, y))| {
                let range = if ax > bx * 3 {
                    (0..=(x / ax)).rev().collect::<Vec<i64>>()
                } else {
                    (0..=(x / ax)).collect::<Vec<i64>>()
                };
                for i in range {
                    let (x, y) = (x - ax * i, y - ay * i);
                    if x % bx == 0 && y % by == 0 && x / bx == y / by {
                        return x / bx + 3 * i;
                    }
                }
                0
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        return "Done with python z3 since Gurobi and rust didnt work".to_string();
    }
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let mut lines = input.lines();
    let mut result = vec![];
    while let Some(line) = lines.next() {
        let line_iter = &mut line.chars();
        let ax = line_iter
            .skip_while(|&c| c != '+')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let ay = line_iter
            .skip_while(|&c| c != '+')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let line = lines.next().unwrap();
        let line_iter = &mut line.chars();
        let bx = line_iter
            .skip_while(|&c| c != '+')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let by = line_iter
            .skip_while(|&c| c != '+')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let line = lines.next().unwrap();
        let line_iter = &mut line.chars();
        let x = line_iter
            .skip_while(|&c| c != '=')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let y = line_iter
            .skip_while(|&c| c != '=')
            .skip(1)
            .take_while(|&c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        result.push(((ax, ay), (bx, by), (x, y)));
        lines.next();
    }
    result
}
