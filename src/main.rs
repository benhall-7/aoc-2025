pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;

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
        Box::new(day_09::Day9),
        Box::new(day_10::Day10),
        Box::new(day_11::Day11),
        Box::new(day_12::Day12),
    ];

    let solution = solutions
        .iter_mut()
        .enumerate()
        .for_each(|(ind, solution)| {
            println!("DAY {}:", ind + 1);
            println!("PART 1");
            solution.problem1();
            println!("PART 2");
            solution.problem2();
        });
}
