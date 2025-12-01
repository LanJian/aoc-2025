use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct SecretEntrance {
    instructions: Vec<i32>,
}

impl FromStr for SecretEntrance {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| {
                let (left, right) = line.split_at(1);
                if left == "L" {
                    right.parse::<i32>().map(|x| -x)
                } else {
                    right.parse::<i32>()
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { instructions })
    }
}

impl SecretEntrance {
    fn password(&self) -> usize {
        let mut ret = 0;
        let mut cur = 50;

        for num in &self.instructions {
            cur = (cur + num).rem_euclid(100);
            if cur == 0 {
                ret += 1;
            }
        }

        ret
    }

    fn new_password(&self) -> usize {
        let mut ret = 0;
        let mut cur = 50;

        for num in &self.instructions {
            if cur == 0 && *num < 0 {
                cur = 100;
            }
            cur += num;
            ret += cur.div_euclid(100).unsigned_abs();
            cur = cur.rem_euclid(100);
            if cur == 0 && *num < 0 {
                ret += 1;
            }
        }

        ret as usize
    }
}

impl Problem for SecretEntrance {
    const DAY: usize = 1;
    const TITLE: &'static str = "secret entrance";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.password())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.new_password())
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
        let solution = SecretEntrance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1158, 6860));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = SecretEntrance::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3, 6));
    }
}
