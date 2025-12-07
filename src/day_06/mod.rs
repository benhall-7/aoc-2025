use nalgebra::DMatrix;

use crate::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn ident(&self) -> i64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    pub fn compute(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn get_problems() -> Vec<(Vec<i64>, Op)> {
    let input = include_str!("input.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().split_whitespace().count();

    let mat = DMatrix::from_row_iterator(
        height,
        width,
        input.lines().flat_map(|line| line.split_whitespace()),
    );
    mat.transpose()
        .row_iter()
        .map(|row| {
            let len = row.len();
            let opstr = row[len - 1];
            let op = match opstr {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => panic!("wrong op! {}", opstr),
            };
            let inputs = row
                .iter()
                .enumerate()
                .take_while(|(ind, _)| *ind < len - 1)
                .map(|(_, cell)| i64::from_str_radix(cell, 10).unwrap())
                .collect();

            (inputs, op)
        })
        .collect()
}

fn get_problems_transposed() -> Vec<(Vec<i64>, Op)> {
    let input = include_str!("input.txt");
    let height = input.lines().count();
    // number of characters in the line is consistent
    let width = input.lines().next().unwrap().len();
    // just get the whole things as a 2x2 block of chars and transpose it
    DMatrix::from_row_iterator(height, width, input.lines().flat_map(|line| line.chars()))
        .transpose()
        .row_iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .split(|s| s.trim().is_empty())
        .map(|problem_set| {
            let len = problem_set[0].len();

            let values = problem_set
                .iter()
                .map(|row| {
                    let numstr = row[0..len - 1].trim();
                    i64::from_str_radix(numstr, 10).unwrap()
                })
                .collect();

            let opstr = &problem_set[0][len - 1..];
            let op = match opstr {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => panic!("wrong op! {}", opstr),
            };

            (values, op)
        })
        .collect()
}

pub struct Day6;

impl Day6 {
    fn solve_problems(&self, problems: Vec<(Vec<i64>, Op)>) -> i64 {
        problems
            .iter()
            .map(|problem| {
                let op = problem.1;
                problem
                    .0
                    .iter()
                    .fold(op.ident(), |acc, value| op.compute(acc, *value))
            })
            .fold(0, |acc, result| acc + result)
    }
}

impl Solution for Day6 {
    fn problem1(&mut self) {
        println!("{}", self.solve_problems(get_problems()));
    }

    fn problem2(&mut self) {
        println!("{}", self.solve_problems(get_problems_transposed()));
    }
}
