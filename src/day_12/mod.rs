use nalgebra::{Matrix3, Vector3};

use crate::Solution;

#[derive(Debug, Clone)]
pub struct Shape {
    pub shape: Matrix3<bool>,
}

#[derive(Debug, Clone)]
pub struct Area {
    pub width: usize,
    pub length: usize,
    pub shape_counts: Vec<usize>,
}

impl Area {
    pub fn is_trivially_possible(&self, num_shapes: usize) -> bool {
        (self.width / 3) * (self.length / 3) >= num_shapes
    }

    pub fn is_trivially_impossible(&self, shapes: Vec<(Shape, usize)>) -> bool {
        let max_space = self.width * self.length;
        let min_space = shapes
            .iter()
            .map(|(shape, count)| shape.shape.iter().map(|c| *c as usize).sum::<usize>() * count)
            .fold(0, |acc, space_per_shape| acc + space_per_shape);
        max_space < min_space
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub shape_defs: Vec<Shape>,
    pub areas: Vec<Area>,
}

impl Config {
    pub fn new_from_file() -> Self {
        let input = include_str!("input.txt");
        let shape_defs = input
            .lines()
            .take_while(|line| !line.contains("x"))
            .collect::<Vec<_>>()
            .split(|line| line.trim().is_empty())
            .filter(|line_group| !line_group.is_empty())
            .map(|lines| {
                let shape_data = lines[1..]
                    .iter()
                    .map(|row| Vector3::from_iterator(row.chars().map(|chr| chr == '#')))
                    .collect::<Vec<_>>();
                let shape: Matrix3<bool> = Matrix3::from_columns(&shape_data);
                Shape { shape }
            })
            .collect();

        let areas = input
            .lines()
            .skip_while(|line| !line.contains("x"))
            .map(|line| {
                let (size, counts) = line.split_once(":").unwrap();
                let (width_str, length_str) = size.split_once("x").unwrap();
                let width = usize::from_str_radix(width_str, 10).unwrap();
                let length = usize::from_str_radix(length_str, 10).unwrap();
                let shape_counts = counts
                    .trim()
                    .split(" ")
                    .map(|cnt| usize::from_str_radix(cnt, 10).unwrap())
                    .collect();

                Area {
                    width,
                    length,
                    shape_counts,
                }
            })
            .collect();

        Config { shape_defs, areas }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Solvable {
    TriviallyYes,
    TriviallyNo,
    Maybe,
}

pub struct Day12;

impl Solution for Day12 {
    fn problem1(&mut self) {
        let config = Config::new_from_file();
        let shapes = config.shape_defs.clone();
        let solvability: Vec<Solvable> = config
            .areas
            .iter()
            .map(|area| {
                let num_shapes = area.shape_counts.iter().fold(0, |acc, count| acc + *count);
                if area.is_trivially_possible(num_shapes) {
                    Solvable::TriviallyYes
                } else if area.is_trivially_impossible(
                    shapes
                        .clone()
                        .into_iter()
                        .zip(area.shape_counts.clone())
                        .collect(),
                ) {
                    Solvable::TriviallyNo
                } else {
                    Solvable::Maybe
                }
            })
            .collect();
        let solvables = solvability
            .iter()
            .filter(|s| **s == Solvable::TriviallyYes)
            .count();
        let unsolvables = solvability
            .iter()
            .filter(|s| **s == Solvable::TriviallyNo)
            .count();
        let maybes = solvability
            .iter()
            .filter(|s| **s == Solvable::Maybe)
            .count();

        println!(
            "solvable: {}, unsolvable: {}, maybe: {}",
            solvables, unsolvables, maybes
        );
    }

    fn problem2(&mut self) {
        // pass
    }
}
