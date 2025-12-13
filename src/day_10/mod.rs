use itertools::Itertools;
use regex::Regex;
use z3::{Optimize, SatResult, ast::Int};

use crate::Solution;

struct Machine {
    light_diagram: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

fn get_machines() -> Vec<Machine> {
    let input = include_str!("input.txt");
    input
        .lines()
        .map(|line| {
            let light_regex = Regex::new(r"\[(.+)\]").unwrap();
            let button_regex = Regex::new(r"\(((\d|,)+)\)").unwrap();
            let joltage_regex = Regex::new(r"\{(.+)\}").unwrap();

            let light_diagram = light_regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .map(|chr| chr == '#')
                .collect();
            let buttons = button_regex
                .captures_iter(line)
                .map(|capt| {
                    capt.get(1)
                        .unwrap()
                        .as_str()
                        .split(",")
                        .map(|num| usize::from_str_radix(num, 10).unwrap())
                        .collect()
                })
                .collect();
            let joltages = joltage_regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|num| u64::from_str_radix(num, 10).unwrap())
                .collect();

            Machine {
                light_diagram,
                buttons,
                joltages,
            }
        })
        .collect()
}

pub struct Day10;

impl Solution for Day10 {
    fn problem1(&mut self) {
        // we don't actually have to count how many presses it takes to achieve the diagram,
        // just which combinations of the buttons "add up" to it (off = sum is even)
        let machines = get_machines();
        let solutions = machines.iter().map(|machine| {
            let max_buttons = machine.buttons.len();
            let config = machine.light_diagram.clone();
            let config_size = config.len();
            for i in 1..max_buttons {
                let found = machine.buttons.iter().combinations(i).find(|button_combo| {
                    let init = vec![false; config_size];
                    let combo_result = button_combo.iter().fold(init, |mut acc, swc| {
                        for ind in *swc {
                            acc[*ind] = !acc[*ind]
                        }
                        acc
                    });

                    combo_result.eq(&config)
                });

                if found.is_some() {
                    return found;
                }
            }
            None
        });

        let sum = solutions.fold(0, |acc, solution| {
            acc + solution.expect("didn't find solution").len()
        });

        println!("{sum}");
    }

    fn problem2(&mut self) {
        let machines = get_machines();
        let solutions = machines.iter().map(|machine| {
            // brute forcing permutations of buttons is a bad idea; our example problems take >300.
            // instead, let's use Z3 to create an optimized solution using the constraints.
            // the variable Bn corresponds to the number of times to press button N
            // the number of equations to satisfy is the length of the joltage array
            // so for example, if we have buttons for (0, 1, 2) and (2)
            // and joltages minimums {5, 8, 20}
            // we get 3 Z3 problems (one for each joltage index):
            // X0      >= 5
            // X0      >= 8
            // X0 + X1 >= 20
            // and we optimize the solutions for the cost eq: X1 + X2
            let optimize = Optimize::new();
            let variables: Vec<_> = (0..machine.buttons.len())
                .into_iter()
                .map(|ind| Int::new_const(format!("B{ind}")))
                .collect();
            for var in &variables {
                optimize.assert(&var.ge(0));
            }
            for (jolt_ind, jolt_min) in machine.joltages.iter().enumerate() {
                let button_vars = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, button)| button.contains(&jolt_ind))
                    .map(|(ind, _)| &variables[ind]);
                // sum up all the button pushes that affect this jolt index
                let equation = button_vars
                    .fold(None, |acc, var| match acc {
                        None => Some(var.clone()),
                        Some(acc) => Some(acc + var),
                    })
                    .expect("expected some buttons to affect this jolt index")
                    // they should add up to at least jolt_min
                    .eq(*jolt_min as i64);
                optimize.assert(&equation);
            }
            let objective = variables
                .iter()
                .fold(None, |acc, var| match acc {
                    None => Some(var.clone()),
                    Some(acc) => Some(acc + var),
                })
                .expect("expected at least 1 variable");
            optimize.minimize(&objective);

            // println!("system:\n{:#?}", optimize);

            match optimize.check(&[]) {
                SatResult::Unknown => panic!(
                    "Solver could not determine satisfiability. Reason: {:?}",
                    optimize.get_reason_unknown()
                ),
                SatResult::Unsat => panic!("Unsatisfiable constraints"),
                SatResult::Sat => {
                    if let Some(model) = optimize.get_model() {
                        return model.eval(&objective, true).unwrap();
                    } else {
                        panic!("Solver returned as satisifable, but did not return a model");
                    }
                }
            }
        });

        let sum = solutions.fold(0i64, |acc, solution| {
            let sol = solution.as_i64().unwrap();
            // println!("{sol}");
            acc + sol
        });
        println!("{sum}")
    }
}
