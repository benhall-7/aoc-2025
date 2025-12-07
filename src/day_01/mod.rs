use crate::Solution;

fn get_turns() -> Vec<i32> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            (if line.chars().nth(0) == Some('R') {
                1
            } else {
                -1
            }) * i32::from_str_radix(&line[1..], 10).expect("couldn't parse number")
        })
        .collect()
}

pub struct Day1;

struct Counter {
    pub value: i32,
    pub count_exacts: i32,
    pub count_passes: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            value: 50,
            count_exacts: 0,
            count_passes: 0,
        }
    }

    pub fn shift(&mut self, shift: i32) {
        let before = self.value;
        let after = before + shift;

        let distance_to_0 = if shift > 0 {
            100 - before
        } else if before > 0 {
            before
        } else {
            100
        };
        let clearance = shift.abs() - distance_to_0;
        let num_passes = (100 + clearance) / 100;

        self.count_passes += num_passes;
        self.value = after.rem_euclid(100);
        if self.value == 0 {
            self.count_exacts += 1;
        }
    }
}

impl Solution for Day1 {
    fn problem1(&mut self) {
        let answer = get_turns().iter().fold(Counter::new(), |mut acc, turn| {
            acc.shift(*turn);
            acc
        });
        println!("{}", answer.count_exacts)
    }

    fn problem2(&mut self) {
        let answer = get_turns().iter().fold(Counter::new(), |mut acc, turn| {
            acc.shift(*turn);
            acc
        });
        println!("{}", answer.count_passes)
    }
}
