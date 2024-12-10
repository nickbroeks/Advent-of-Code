use crate::day::Day;

pub struct Day10;

impl Day for Day10 {
    fn part1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let mut sum = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    continue;
                }

                let mut ends = vec![(i, j)];
                let mut height = 0;
                while !ends.is_empty() && height != 9 {
                    height += 1;
                    let mut temp = vec![];
                    for (i, j) in ends {
                        if i > 0 && grid[i - 1][j] == height && !temp.contains(&(i - 1, j)) {
                            temp.push((i - 1, j));
                        }
                        if i < grid.len() - 1
                            && grid[i + 1][j] == height
                            && !temp.contains(&(i + 1, j))
                        {
                            temp.push((i + 1, j));
                        }
                        if j > 0 && grid[i][j - 1] == height && !temp.contains(&(i, j - 1)) {
                            temp.push((i, j - 1));
                        }
                        if j < grid[0].len() - 1
                            && grid[i][j + 1] == height
                            && !temp.contains(&(i, j + 1))
                        {
                            temp.push((i, j + 1));
                        }
                    }
                    ends = temp;
                }
                sum += ends.len();
            }
        }
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        exec(input, true).to_string()
    }
}
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn exec(input: &str, count_unique: bool) -> usize {
    let grid = parse_input(input);
    let mut sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 0 {
                continue;
            }

            let mut ends = vec![(i, j)];
            let mut height = 0;
            while !ends.is_empty() && height != 9 {
                height += 1;
                let mut temp = vec![];
                for (i, j) in ends {
                    if i > 0
                        && grid[i - 1][j] == height
                        && (count_unique || !temp.contains(&(i - 1, j)))
                    {
                        temp.push((i - 1, j));
                    }
                    if i < grid.len() - 1
                        && grid[i + 1][j] == height
                        && (count_unique || !temp.contains(&(i + 1, j)))
                    {
                        temp.push((i + 1, j));
                    }
                    if j > 0
                        && grid[i][j - 1] == height
                        && (count_unique || !temp.contains(&(i, j - 1)))
                    {
                        temp.push((i, j - 1));
                    }
                    if j < grid[0].len() - 1
                        && grid[i][j + 1] == height
                        && (count_unique || !temp.contains(&(i, j + 1)))
                    {
                        temp.push((i, j + 1));
                    }
                }
                ends = temp;
            }
            sum += ends.len();
        }
    }
    sum
}
