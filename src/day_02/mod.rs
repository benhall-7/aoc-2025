use std::{collections::HashSet, i64};

use crate::Solution;

fn get_ranges() -> Vec<(i64, i64)> {
    let input = include_str!("input.txt");
    input
        .split(",")
        .map(|range| {
            let (part_1, part_2) = range.split_once("-").unwrap();
            let lower = i64::from_str_radix(part_1, 10).unwrap();
            let upper = i64::from_str_radix(part_2, 10).unwrap();
            (lower, upper)
        })
        .collect()
}

pub struct Day2;

impl Solution for Day2 {
    fn problem1(&mut self) {
        let sum = get_ranges().iter().fold(0, |total_sum, (lower, upper)| {
            let num_digits_lower = lower.checked_ilog10().unwrap() + 1;
            let num_digits_upper = upper.checked_ilog10().unwrap() + 1;

            let all_matches = (num_digits_lower..=num_digits_upper)
                .filter(|digit_num| digit_num % 2 == 0)
                .flat_map(|even| {
                    // for even digits, we only care about the first half
                    // 125114-804265
                    // 3 digit halves
                    // 125 to 804, is the range of possible hits,
                    // just go through and add each one to the list
                    let half_digits = even / 2;
                    // e.g. 1_000
                    let factor = 10i64.pow(half_digits);
                    // e.g. 100_000 or 125114
                    let lower_bound = 10i64.pow(even - 1).max(*lower);
                    // e.g. 999_999 or 804265
                    let upper_bound = (10i64.pow(even) - 1).min(*upper);
                    // e.g. 125
                    let lower_first_half = lower_bound / factor;
                    // e.g. 804
                    let upper_first_half = upper_bound / factor;
                    // go from lower to upper and add any matches
                    (lower_first_half..=upper_first_half)
                        .map(move |first_half| first_half * (factor + 1))
                        .filter(|possible_match| possible_match >= lower && possible_match <= upper)
                });

            total_sum + all_matches.fold(0, |acc, sum_per| acc + sum_per)
        });

        println!("{sum}");
    }

    fn problem2(&mut self) {
        let sum = get_ranges().iter().fold(0, |total_sum, (lower, upper)| {
            let num_digits_lower = lower.checked_ilog10().unwrap() + 1;
            let num_digits_upper = upper.checked_ilog10().unwrap() + 1;

            let all_matches = (2..=num_digits_upper).flat_map(|repeats| {
                let matches = (num_digits_lower..=num_digits_upper)
                    .filter(move |digit_num| digit_num % repeats == 0)
                    // divisible, meaning multiple of 3, like 3, 6, 9, etc
                    .flat_map(move |digit_num| {
                        // let's consider 3 repeats
                        // 125114-804265
                        // 2 digit thirds
                        // 12 to 80, is the range of possible hits,
                        // just go through and add each one to the list
                        // e.g. repeats = 3 times in 6 digits,
                        //      so we're looking for repeats of 2 digit numbers
                        let chunk_digits = digit_num / repeats;
                        // e.g. 1_00_00
                        let factor = 10i64.pow(digit_num - chunk_digits);
                        // e.g. 100000 or 125114
                        let lower_bound = 10i64.pow(digit_num - 1).max(*lower);
                        // e.g. 999999 or 804265
                        let upper_bound = (10i64.pow(digit_num) - 1).min(*upper);
                        // e.g. 125
                        let lower_first_part = lower_bound / factor;
                        // e.g. 804
                        let upper_first_part = upper_bound / factor;
                        // e.g. 010101
                        // compute by iterating 3 (repeat) times
                        // we take 10^0 + 10^2 + 10^4
                        let pattern = (0..repeats)
                            .fold(0, |acc, position| acc + 10i64.pow(position * chunk_digits));
                        // go from lower to upper and add any matches
                        (lower_first_part..=upper_first_part)
                            .map(move |first_part| first_part * pattern)
                            .filter(|possible_match| {
                                possible_match >= lower && possible_match <= upper
                            })
                    });

                matches
            });

            total_sum
                + all_matches
                    .collect::<HashSet<_>>()
                    .iter()
                    .fold(0, |acc, sum_per| acc + sum_per)
        });

        println!("{sum}");
    }
}
