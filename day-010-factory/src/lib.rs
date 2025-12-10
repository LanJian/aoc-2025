use anyhow::Ok;
use good_lp::Solution;
use good_lp::SolverModel;
use good_lp::default_solver;
use std::str::FromStr;

use anyhow::{Result, anyhow, bail};
use aoc_plumbing::Problem;
use good_lp::Expression;
use good_lp::ProblemVariables;
use good_lp::variable;
use itertools::Itertools;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: [usize; 16],
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut target = 0;
        let mut buttons = Vec::default();
        let mut joltages = [0; 16];

        for token in s.split_whitespace() {
            let bytes = token.as_bytes();

            match token.as_bytes()[0] {
                b'[' => bytes[1..bytes.len() - 1]
                    .iter()
                    .enumerate()
                    .filter(|&(_, c)| *c == b'#')
                    .for_each(|(i, _)| target |= 1 << i),
                b'(' => {
                    let mut button = 0;
                    for x in token[1..bytes.len() - 1].split(',') {
                        let num = x.parse::<usize>()?;
                        let mask = 1 << num;
                        button |= mask;
                    }
                    buttons.push(button);
                }
                b'{' => {
                    for (i, x) in token[1..bytes.len() - 1].split(',').enumerate() {
                        joltages[i] = x.parse::<usize>()?;
                    }
                }
                _ => bail!("Could not parse machine"),
            }
        }

        Ok(Self {
            target,
            buttons,
            joltages,
        })
    }
}

impl Machine {
    fn configure_indicators(&self) -> Result<usize> {
        (1..=self.buttons.len())
            .find(|&k| {
                self.buttons
                    .iter()
                    .combinations(k)
                    .any(|x| x.iter().fold(0, |acc, &item| acc ^ *item) == self.target)
            })
            .ok_or_else(|| {
                anyhow!("Could not find a combination of buttons to configure indicators")
            })
    }

    fn configure_joltages(&self) -> Result<usize> {
        let mut problem = ProblemVariables::new();
        let vars = problem.add_vector(variable().integer().min(0), self.buttons.len());
        let objective: Expression = vars.iter().sum();
        let mut model = problem.minimise(objective).using(default_solver);

        for (i, &target) in self.joltages.iter().enumerate() {
            let mut constraint_expr = Expression::with_capacity(1);
            for (j, &button) in self.buttons.iter().enumerate() {
                if button & (1 << i) > 0 {
                    constraint_expr += vars[j];
                }
            }

            model = model.with(constraint_expr.eq(Expression::from_other_affine(target as u32)));
        }

        let solution = model.solve()?;

        let total = vars
            .iter()
            .map(|&x| solution.value(x).round() as usize)
            .sum();

        Ok(total)
    }
}

#[derive(Debug, Clone)]
pub struct Factory {
    machines: Vec<Machine>,
}

impl FromStr for Factory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let machines = s.lines().map(Machine::from_str).collect::<Result<_, _>>()?;
        Ok(Self { machines })
    }
}

impl Problem for Factory {
    const DAY: usize = 10;
    const TITLE: &'static str = "factory";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        self.machines
            .iter()
            .try_fold(0, |acc, x| x.configure_indicators().map(|v| acc + v))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        self.machines
            .iter()
            .try_fold(0, |acc, x| x.configure_joltages().map(|v| acc + v))
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = Factory::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(449, 17848));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Factory::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(7, 33));
    }
}
