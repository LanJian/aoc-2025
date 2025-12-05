use std::{cmp::Ordering, str::FromStr};

use anyhow::anyhow;
use aoc_common::interval::Intervals;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct Cafeteria {
    ranges: Intervals,
    ingredients: Vec<isize>,
}

impl FromStr for Cafeteria {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = Intervals::default();
        let (part1, part2) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("could not parse input"))?;

        for l in part1.lines() {
            let (s, e) = l
                .split_once('-')
                .ok_or_else(|| anyhow!("could not parse range"))?;
            ranges.add((s.parse()?, e.parse::<isize>()? + 1).into());
        }

        let ingredients = part2
            .lines()
            .map(|x| x.parse::<isize>())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            ranges,
            ingredients,
        })
    }
}

impl Cafeteria {
    fn binary_search(&self, i: isize) -> bool {
        // intervals are already disjoint and sorted, so we can bin search
        self.ranges
            .intervals
            .binary_search_by(|x| {
                if x.start > i {
                    Ordering::Greater
                } else if x.end <= i {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
    }
}

impl Problem for Cafeteria {
    const DAY: usize = 5;
    const TITLE: &'static str = "cafeteria";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let ret = self
            .ingredients
            .iter()
            .filter(|&i| self.binary_search(*i))
            .count();

        Ok(ret)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.ranges.len())
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
        let solution = Cafeteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(640, 365804144481581));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Cafeteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3, 14));
    }
}
