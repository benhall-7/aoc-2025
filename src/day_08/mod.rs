use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra::Vector3;

use crate::Solution;

fn get_coordinates() -> Vec<Vector3<i64>> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            let coords: Vec<_> = line
                .split(",")
                .map(|num| i64::from_str_radix(num, 10).unwrap())
                .collect();
            Vector3::new(coords[0], coords[1], coords[2])
        })
        .collect()
}

pub struct Day8;

impl Solution for Day8 {
    fn problem1(&mut self) {
        // it's n^2 to get all the distances
        // not sure if there's a way to improve it other than oct-trees
        let coords = get_coordinates();
        let len = coords.len();
        let unique_pairs: Vec<(usize, usize)> = (0..len)
            .into_iter()
            .cartesian_product((0..len).into_iter())
            .filter(|(i, j)| i < j)
            .collect();

        let edges: Vec<(usize, usize)> = unique_pairs
            .iter()
            .map(|pair| {
                let a_f64 = coords[pair.0].map(|x| x as f64);
                let b_f64 = coords[pair.1].map(|x| x as f64);
                (*pair, a_f64.metric_distance(&b_f64))
            })
            .sorted_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(pair, _)| pair)
            .take(1000) // "connect the 1000 closest pairs"
            .collect();

        let mut graph: HashMap<usize, HashSet<usize>> = Default::default();
        edges.iter().for_each(|&(a, b)| {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
        });

        let mut groups: Vec<HashSet<usize>> = Default::default();
        while !graph.is_empty() {
            let mut group: HashSet<usize> = Default::default();
            let arbitrary = graph.keys().next().unwrap();
            let mut to_remove = vec![*arbitrary];
            while let Some(popped) = to_remove.pop() {
                let possible = graph.remove(&popped);
                if let Some(neighbors) = possible {
                    to_remove.append(&mut neighbors.into_iter().collect());
                    group.insert(popped);
                }
            }
            groups.push(group);
        }

        let sizes: Vec<usize> = groups
            .iter()
            .map(|group| group.len())
            .sorted()
            .rev()
            .collect();

        let answer = sizes[0] * sizes[1] * sizes[2];

        println!("{:#?}", answer);
    }

    fn problem2(&mut self) {
        let coords = get_coordinates();
        let len = coords.len();
        let unique_pairs: Vec<(usize, usize)> = (0..len)
            .into_iter()
            .cartesian_product((0..len).into_iter())
            .filter(|(i, j)| i < j)
            .collect();

        let edges: Vec<(usize, usize)> = unique_pairs
            .iter()
            .map(|pair| {
                let a_f64 = coords[pair.0].map(|x| x as f64);
                let b_f64 = coords[pair.1].map(|x| x as f64);
                (*pair, a_f64.metric_distance(&b_f64))
            })
            .sorted_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(pair, _)| pair)
            .collect();

        // every box starts as its own unconnected graph (aka groups)
        // connections between different groups merge the two into one
        // we're done when there's only one group left.
        // we don't actually need to know how they're connected within a group
        // a HashSet should be perfect to test connectivity
        let mut groups: Vec<HashSet<usize>> = (0..len).map(|id| HashSet::from([id])).collect();
        let last_edge = edges
            .iter()
            .find(|&&(a, b)| {
                // a must be present somewhere. unwrap is safe here
                let (index_a, ..) = groups
                    .iter()
                    .find_position(|group| group.contains(&a))
                    .unwrap();
                // only take action when a and b are in different groups
                if !groups[index_a].contains(&b) {
                    // take out the group with a
                    let set_a = groups.remove(index_a);
                    // find which group b is in
                    let (index_b, ..) = groups
                        .iter()
                        .find_position(|group| group.contains(&b))
                        .unwrap();
                    // take out the group with b
                    let set_b = groups.remove(index_b);
                    // merge the two groups
                    let merged = &set_a | &set_b;
                    // put them back in the list
                    groups.push(merged);
                }

                groups.len() == 1
            })
            .unwrap();

        let answer = coords[last_edge.0].x * coords[last_edge.1].x;

        println!("{answer}");
    }
}
