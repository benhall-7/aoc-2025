use crate::Solution;

fn get_banks() -> Vec<Vec<u64>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| {
                    let mut tmp = [0u8; 4];
                    u64::from_str_radix(chr.encode_utf8(&mut tmp), 10).unwrap()
                })
                .collect()
        })
        .collect()
}

pub struct Day3;

impl Solution for Day3 {
    fn problem1(&mut self) {
        let sum = get_banks()
            .iter()
            .map(|bank| {
                let len = bank.len();
                let (ind1, num1) = bank[0..(len - 1)].iter().enumerate().fold(
                    (0usize, 0u64),
                    |acc, (ind, &jolt)| {
                        if jolt > acc.1 { (ind, jolt) } else { acc }
                    },
                );
                let (_ind2, num2) = bank[(ind1 + 1)..len].iter().enumerate().fold(
                    (0usize, 0u64),
                    |acc, (ind, &jolt)| {
                        if jolt > acc.1 { (ind, jolt) } else { acc }
                    },
                );
                num1 * 10 + num2
            })
            .fold(0, |acc, val| acc + val);

        println!("{sum}")
    }

    fn problem2(&mut self) {
        let sum = get_banks()
            .iter()
            .map(|bank| {
                let len = bank.len();

                (0..12)
                    .rev()
                    .fold((0usize, 0), |(start_index, total), dig| {
                        let (ind, max_num) =
                            bank[start_index..(len - dig)].iter().enumerate().fold(
                                (0usize, 0u64),
                                |acc, (ind, &jolt)| {
                                    if jolt > acc.1 { (ind, jolt) } else { acc }
                                },
                            );

                        let next_index = start_index + ind + 1;
                        let digit_part = max_num * 10u64.pow(dig as u32);
                        (next_index, total + digit_part)
                    })
            })
            .fold(0, |acc, (_, bank_joltage)| acc + bank_joltage);

        println!("{sum}")
    }
}
