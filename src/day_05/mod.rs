use std::collections::{BTreeMap, HashSet};

use crate::Solution;

fn get_ranges() -> Vec<(i64, i64)> {
    let ranges = include_str!("input_ranges.txt");
    ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                i64::from_str_radix(start, 10).unwrap(),
                i64::from_str_radix(end, 10).unwrap(),
            )
        })
        .collect()
}

fn get_ids() -> Vec<i64> {
    let ids = include_str!("input_ids.txt");
    ids.lines()
        .map(|line| i64::from_str_radix(line, 10).unwrap())
        .collect()
}

pub struct Day5;

impl Solution for Day5 {
    fn problem1(&mut self) {
        let ranges = get_ranges();
        let ids = get_ids();

        // it takes like 3 lines for the n^2 search but why not do it more optimally?
        let start_tree = ranges
            .iter()
            .map(|range| (range.0, range))
            .collect::<BTreeMap<_, _>>();
        let end_tree = ranges
            .iter()
            .map(|range| (range.1, range))
            .collect::<BTreeMap<_, _>>();

        let num_matches = ids
            .iter()
            .filter(|&&id| {
                let valid_starts = start_tree
                    .range(..=id)
                    .map(|(_, rng)| **rng)
                    .collect::<HashSet<_>>();
                let valid_ends = end_tree
                    .range(id..)
                    .map(|(_, rng)| **rng)
                    .collect::<HashSet<_>>();
                let possible_ranges = valid_starts.union(&valid_ends);

                possible_ranges
                    .into_iter()
                    .any(|range| range.0 <= id && range.1 >= id)
            })
            .count();

        println!("{num_matches}");
    }

    fn problem2(&mut self) {
        let mut ranges = get_ranges();
        // sort by start value
        ranges.sort_by_key(|rng| rng.0);
        // remove redundancies by finding overlaps
        let mut i = 0;
        while i < ranges.len() {
            let nx = i + 1;
            // if the next range falls within the current range, merge them
            while i < ranges.len() - 1 && ranges[nx].0 <= ranges[i].1 {
                // current range end can become extended, and other range is removed
                ranges[i].1 = ranges[i].1.max(ranges[nx].1);
                ranges.remove(nx);
            }

            i += 1;
        }
        let size = ranges
            .iter()
            .map(|r| r.1 - r.0 + 1)
            .fold(0, |acc, span| acc + span);

        println!("{size}");
    }
}
