pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;

pub trait Solution {
    fn problem1(&mut self) {
        println!("TODO");
    }
    fn problem2(&mut self) {
        println!("TODO");
    }
}

fn main() {
    let mut solutions: [Box<dyn Solution>; _] = [
        Box::new(day_01::Day1),
        Box::new(day_02::Day2),
        Box::new(day_03::Day3),
        Box::new(day_04::Day4),
        Box::new(day_05::Day5),
        Box::new(day_06::Day6),
        Box::new(day_07::Day7),
        Box::new(day_08::Day8),
    ];

    let solution = solutions[..].iter_mut().last().unwrap();

    solution.problem1();
    solution.problem2();
}
