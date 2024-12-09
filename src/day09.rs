use crate::{day::Day, util::Pipe};

pub struct Day09;

impl Day for Day09 {
    fn part1(&self, input: &str) -> String {
        parse_input(input).pipe(|disk| {
            let updated_disk: Vec<(i32, u32)> = disk
                .iter()
                .enumerate()
                .map(|(i, &count)| {
                    if i % 2 == 0 {
                        ((i / 2) as i32, count)
                    } else {
                        (-1, count)
                    }
                })
                .collect();
            let mut reverse_iter = updated_disk.iter().rev();
            let mut iter = updated_disk.iter();
            let mut sum = 0;
            let mut index = 0;
            let (mut back_id, mut back_count) = *reverse_iter.next().unwrap();
            let (mut id, mut count) = *iter.next().unwrap();
            loop {
                if id == back_id {
                    for i in 0..back_count {
                        sum += back_id as u64 * (index + i) as u64;
                    }
                    break;
                } else if id == -1 {
                    let n = count;
                    (id, count) = *iter.next().unwrap();
                    if back_id == id {
                        continue;
                    }
                    for i in 0..n {
                        sum += back_id as u64 * (index + i) as u64;
                        back_count -= 1;
                        if back_count == 0 {
                            reverse_iter.next();
                            (back_id, back_count) = *reverse_iter.next().unwrap();
                            if back_id == id {
                                index += i;
                                break;
                            }
                        }
                    }
                    index += n;
                } else if id < back_id {
                    for i in 0..count {
                        sum += id as u64 * (index + i) as u64;
                    }
                    index += count;
                    (id, count) = *iter.next().unwrap();
                } else {
                    println!("Error! {} {}", id, back_id);
                    break;
                }
            }
            sum.to_string()
        })
    }

    fn part2(&self, input: &str) -> String {
        parse_input(input).pipe(|disk| {
            let mut updated_disk: Vec<(i32, usize)> = disk
                .iter()
                .enumerate()
                .map(|(i, &count)| {
                    if i % 2 == 0 {
                        ((i / 2) as i32, count as usize)
                    } else {
                        (-1, count as usize)
                    }
                })
                .collect();
            let temp = updated_disk.clone();
            let reverse_iter = temp.iter().rev();
            for &(last_id, last_count) in reverse_iter {
                let mut i = 0;
                while updated_disk[i].0 != last_id
                    && (updated_disk[i].0 != -1 || updated_disk[i].1 < last_count)
                {
                    i += 1;
                }
                if updated_disk[i].0 == last_id {
                    continue;
                }
                let left = updated_disk[i].1 - last_count;
                let mut j = 0;
                while updated_disk[j].0 != last_id {
                    j += 1;
                }
                updated_disk[j] = (-1, last_count);
                updated_disk[i] = (last_id, last_count);
                if left > 0 {
                    updated_disk.insert(i + 1, (-1, left));
                }
            }
            // updated_disk
            //     .iter()
            //     .fold("".to_string(), |mut acc, &(id, count)| {
            //         for _i in 0..count {
            //             acc = if id == -1 {
            //                 acc + &"_".to_string()
            //             } else {
            //                 acc + &id.to_string()
            //             };
            //         }
            //         acc
            //     })
            //     .to_string()
            updated_disk
                .iter()
                .fold((0, 0), |(index, sum), &(id, count)| {
                    (
                        index + count,
                        if id == -1 {
                            sum as i64
                        } else {
                            sum as i64
                                + id as i64
                                    * ((2 * index as i64 + count as i64 - 1) * (count as i64) / 2)
                        },
                    )
                })
                .1
                .to_string()
        })
    }
}
fn parse_input(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
