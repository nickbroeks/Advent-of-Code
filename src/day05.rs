use crate::{day::Day, util::Pipe};

pub struct Day05;

impl Day for Day05 {
    fn part1(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);
        updates
            .iter()
            .filter(|&update| {
                let used_rules: Vec<&(u32, u32)> = rules
                    .iter()
                    .filter(|&rule| update.contains(&rule.0) && update.contains(&rule.1))
                    .collect();
                let mut seen_pages = vec![];
                for &page in update.iter() {
                    if used_rules
                        .iter()
                        .filter(|&rule| rule.0 == page)
                        .any(|rule| seen_pages.contains(&rule.1))
                    {
                        return false;
                    }
                    seen_pages.push(page);
                }
                true
            })
            .map(|x| x[(x.len() - 1) / 2])
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (rules, updates) = parse_input(input);
        updates
            .iter()
            .filter(|&update| {
                let used_rules: Vec<&(u32, u32)> = rules
                    .iter()
                    .filter(|&rule| update.contains(&rule.0) && update.contains(&rule.1))
                    .collect();
                let mut seen_pages = vec![];
                for &page in update.iter() {
                    if used_rules
                        .iter()
                        .filter(|&rule| rule.0 == page)
                        .any(|rule| seen_pages.contains(&rule.1))
                    {
                        return true;
                    }
                    seen_pages.push(page);
                }
                false
            })
            .map(|update| {
                let mut used_rules: Vec<&(u32, u32)> = rules
                    .iter()
                    .filter(|&rule| update.contains(&rule.0) && update.contains(&rule.1))
                    .collect();
                let mut pages = vec![];
                let mut not_seen_pages = update.clone();
                while used_rules.len() > 0 {
                    let page = *not_seen_pages
                        .iter()
                        .find(|&&page| used_rules.iter().all(|&rule| rule.1 != page))
                        .unwrap();
                    not_seen_pages = not_seen_pages
                        .iter()
                        .filter(|&&p| page != p)
                        .map(|&x| x)
                        .collect();
                    used_rules = used_rules
                        .iter()
                        .filter(|&&&rule| rule.0 != page)
                        .map(|&rule| rule)
                        .collect();
                    pages.push(page);
                }
                pages.extend(not_seen_pages);
                pages
            })
            .map(|x| x[(x.len() - 1) / 2])
            .sum::<u32>()
            .to_string()
        // .map(|x| {
        //     x.iter()
        //         .map(|y| y.to_string())
        //         .collect::<Vec<String>>()
        //         .join(" ")
        // })
        // .collect::<Vec<String>>()
        // .join("\n")
    }
}
fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut iter = input.lines();
    let rules = iter
        .by_ref()
        .map_while(|x| match x {
            "" => None,
            _ => Some(
                x.split("|")
                    .collect::<Vec<&str>>()
                    .pipe(|pair| (pair[0].parse().unwrap(), pair[1].parse().unwrap())),
            ),
        })
        .collect();
    let updates = iter
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}
