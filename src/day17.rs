use crate::day::Day;
pub struct Day17;

impl Day for Day17 {
    fn part1(&self, input: &str) -> String {
        let (a, b, c, program) = parse_input(input);
        let output = run_program(a, b, c, &program);
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part2(&self, input: &str) -> String {
        let (_, b, c, program) = parse_input(input);
        let program_len = program.len();

        // Start by trying to match the last 4 output digits first
        let mut match_length = 4;

        // Start with the smallest number that satisfies match_length 3-bit words
        let mut candidate_a = 1 << (3 * (match_length - 1));

        while match_length <= program_len {
            let expected_output = &program[(program_len - match_length)..];
            while !is_output_matching(candidate_a, b, c, &program, expected_output) {
                candidate_a += 1;
            }
            // Increment match_length and shift candidate_a for the next match attempt
            match_length += 1;
            candidate_a <<= 3;
        }
        // Return the final value of candidate_a adjusted for the last shift
        (candidate_a >> 3).to_string()
    }
}

fn is_output_matching(a: i64, b: i64, c: i64, program: &Vec<u8>, expected_output: &[u8]) -> bool {
    let output = run_program(a, b, c, program);
    output.len() == expected_output.len()
        && output
            .iter()
            .zip(expected_output.iter())
            .all(|(&o, &e)| o == e)
}

fn run_program(mut a: i64, mut b: i64, mut c: i64, program: &Vec<u8>) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;
    while ip < program.len() {
        let literal = program[ip as usize + 1] as i64;
        let get_combo = || match literal {
            4 => a,
            5 => b,
            6 => c,
            7 => panic!("Invalid combo value encountered"),
            x => x,
        };
        match program[ip] {
            0 => a >>= get_combo(),
            1 => b ^= literal,
            2 => b = get_combo() & 7,
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => output.push((get_combo() & 7) as u8),
            6 => b = a >> get_combo(),
            7 => c = a >> get_combo(),
            _ => panic!("Unknown opcode encountered"),
        }
        ip += 2;
    }
    return output;
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<u8>) {
    let mut lines = input.lines();
    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());
    lines.next();
    let program = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    (a, b, c, program)
}
fn parse_register(line: &str) -> i64 {
    line.split(": ").nth(1).unwrap().parse().unwrap()
}
