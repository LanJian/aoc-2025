use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct TrashCompactor {
    human_solution: u64,
    cephalopod_solution: u64,
}

impl FromStr for TrashCompactor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut human_problems = Vec::default();
        let mut cephalopod_problems = Vec::default();
        let mut human_solution = 0_u64;
        let mut cephalopod_solution = 0_u64;

        for (i, line) in s.lines().enumerate() {
            let mut jj = 0;
            let mut whitespace = true;
            for (col, c) in line.chars().enumerate() {
                if c.is_whitespace() {
                    whitespace = true;
                } else if c.is_ascii_digit() {
                    if whitespace {
                        jj += 1;
                        whitespace = false;
                    }

                    let digit = c.to_digit(10).ok_or_else(|| anyhow!("Invalid digit"))? as u64;
                    let j = jj - 1;

                    if human_problems.len() <= j {
                        human_problems.push(Vec::default());
                    }
                    if human_problems[j].len() <= i {
                        human_problems[j].push(0);
                    }
                    human_problems[j][i] = human_problems[j][i] * 10 + digit;

                    while cephalopod_problems.len() <= col {
                        cephalopod_problems.push(0);
                    }
                    cephalopod_problems[col] = cephalopod_problems[col] * 10 + digit;
                } else if c == '+' {
                    if whitespace {
                        jj += 1;
                        whitespace = false;
                    }

                    let j = jj - 1;
                    human_solution += human_problems[j].iter().sum::<u64>();

                    let mut sum = 0;
                    let mut cc = col;
                    while cc < cephalopod_problems.len() && cephalopod_problems[cc] != 0 {
                        sum += cephalopod_problems[cc];
                        cc += 1;
                    }
                    cephalopod_solution += sum;
                } else if c == '*' {
                    if whitespace {
                        jj += 1;
                        whitespace = false;
                    }

                    let j = jj - 1;
                    human_solution += human_problems[j].iter().product::<u64>();

                    let mut product = 1;
                    let mut cc = col;
                    while cc < cephalopod_problems.len() && cephalopod_problems[cc] != 0 {
                        product *= cephalopod_problems[cc];
                        cc += 1;
                    }
                    cephalopod_solution += product;
                }
            }
        }

        Ok(Self {
            human_solution,
            cephalopod_solution,
        })
    }
}

impl Problem for TrashCompactor {
    const DAY: usize = 6;
    const TITLE: &'static str = "trash compactor";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.human_solution)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.cephalopod_solution)
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
        let solution = TrashCompactor::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4405895212738, 7450962489289));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = TrashCompactor::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4277556, 3263827));
    }
}
