use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
use aoc_plumbing::Problem;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone)]
struct BatteryBank {
    batteries: Vec<u64>,
}

impl FromStr for BatteryBank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let batteries = s
            .chars()
            .map(|x| x.to_digit(10).map(|d| d as u64))
            .collect::<Option<_>>()
            .ok_or_else(|| anyhow!("Could not parse battery bank"))?;
        Ok(Self { batteries })
    }
}

impl BatteryBank {
    fn joltages(&self, count: usize) -> Vec<u64> {
        let n = self.batteries.len();
        let m = count;
        let mut dp = vec![0; m + 1];

        for i in 1..=n {
            let mut new_dp = vec![0; m + 1];

            for j in 1..=m {
                new_dp[j] = dp[j].max(dp[j - 1] * 10 + self.batteries[i - 1]);
            }

            dp = new_dp;
        }

        dp
    }
}

#[derive(Debug, Clone)]
pub struct Lobby {
    part1: u64,
    part2: u64,
}

impl FromStr for Lobby {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let battery_banks: Vec<_> = s
            .lines()
            .map(BatteryBank::from_str)
            .collect::<Result<_>>()?;

        let sums = battery_banks
            .par_iter()
            .map(|x| x.joltages(12))
            .map(|x| (x[2], x[12]))
            .reduce(|| (0, 0), |(a, b), (c, d)| (a + c, b + d));

        Ok(Self {
            part1: sums.0,
            part2: sums.1,
        })
    }
}

impl Lobby {
    fn total_small_joltage(&self) -> u64 {
        self.part1
    }

    fn total_large_joltage(&self) -> u64 {
        self.part2
    }
}

impl Problem for Lobby {
    const DAY: usize = 3;
    const TITLE: &'static str = "lobby";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.total_small_joltage())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.total_large_joltage())
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
        let solution = Lobby::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(17493, 173685428989126));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Lobby::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(357, 3121910778619));
    }
}
