use std::collections::HashMap;

use crate::Solution;

struct Graph {
    graph: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_file() -> Self {
        let input = include_str!("input.txt");
        let graph = input
            .lines()
            .map(|line| {
                let (first, second) = line.split_once(":").unwrap();
                (
                    first.to_string(),
                    second
                        .trim()
                        .split(" ")
                        .map(String::from)
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        Self { graph }
    }

    // the problem doesn't suggest this could be the case,
    // but this only works if the paths are not cyclical
    fn count_paths<const L: usize>(
        &self,
        node: String,
        target: &'static str,
        mut visit_target: [(&'static str, bool); L],
        cache: &mut HashMap<(String, &'static str, [(&'static str, bool); L]), u64>,
    ) -> u64 {
        if let Some(cached) = cache.get(&(node.clone(), target, visit_target)) {
            return *cached;
        }

        let ret = if node == target {
            visit_target.iter().all(|target| target.1) as u64
        } else {
            visit_target
                .iter_mut()
                .for_each(|target| target.1 |= target.0 == node);
            self.graph
                .get(&node)
                .unwrap_or(&vec![])
                .iter()
                .fold(0, |paths, child| {
                    paths + self.count_paths(child.clone(), target, visit_target, cache)
                })
        };
        cache.insert((node, target, visit_target), ret);
        ret
    }
}

pub struct Day11;

impl Solution for Day11 {
    fn problem1(&mut self) {
        let graph = Graph::from_file();
        let paths = graph.count_paths(String::from("you"), "out", [], &mut Default::default());
        println!("{paths}");
    }

    fn problem2(&mut self) {
        let graph = Graph::from_file();
        let paths = graph.count_paths(
            String::from("svr"),
            "out",
            [("dac", false), ("fft", false)],
            &mut Default::default(),
        );
        println!("{paths}");
    }
}
