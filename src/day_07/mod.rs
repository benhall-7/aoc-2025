use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::Solution;

#[derive(Debug, Clone)]
struct Config {
    start: usize,
    splitter_grid: Vec<Vec<usize>>,
}

impl Config {
    pub fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let mut start = 0;
        let splitter_grid = input
            .lines()
            .map(|line| {
                line.char_indices()
                    .filter_map(|(ind, chr)| match chr {
                        'S' => {
                            start = ind;
                            None
                        }
                        '^' => Some(ind),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Self {
            start,
            splitter_grid,
        }
    }
}

pub struct Day7;

impl Day7 {
    fn num_paths(
        &self,
        y: usize,
        x: usize,
        grid: &Vec<Vec<usize>>,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        // check cache
        if let Some(res) = cache.get(&(y, x)) {
            return *res;
        }
        // search down for the next split from the initial (y, x) position
        let split_search = grid[y..].iter().find_position(|row| row.contains(&x));

        let ret = if let Some((y_offset, _)) = split_search {
            // sum both paths if found
            let new_y = y + y_offset;
            self.num_paths(new_y, x - 1, grid, cache) + self.num_paths(new_y, x + 1, grid, cache)
        } else {
            // otherwise, it's just one path
            1
        };

        // add to cache
        cache.insert((y, x), ret);
        ret
    }
}

impl Solution for Day7 {
    fn problem1(&mut self) {
        let Config {
            start,
            splitter_grid,
        } = Config::new_from_file();
        let mut beams = HashSet::from([start]);
        let mut splits = 0;
        for splitters in &splitter_grid {
            for splitter in splitters {
                if beams.remove(splitter) {
                    beams.insert(*splitter - 1);
                    beams.insert(*splitter + 1);
                    splits += 1;
                }
            }
        }

        println!("{splits}");
    }

    fn problem2(&mut self) {
        let Config {
            start,
            splitter_grid,
        } = Config::new_from_file();

        let splits = self.num_paths(0, start, &splitter_grid, &mut Default::default());

        println!("{splits}");
    }
}
