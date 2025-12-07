pub mod day_01;
pub mod day_02;
pub mod day_03;

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
    ];

    let solution = solutions[..].iter_mut().last().unwrap();

    solution.problem1();
    solution.problem2();
}
