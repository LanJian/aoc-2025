use std::{collections::VecDeque, str::FromStr};

use aoc_common::grid::Grid;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct PrintingDepartment {
    grid: Grid<char>,
}

impl FromStr for PrintingDepartment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl PrintingDepartment {
    fn accessible_paper(&self) -> usize {
        let mut ret = 0;

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();
                if self.grid[coord] == '@'
                    && coord
                        .neighbours()
                        .into_iter()
                        .filter(|&x| self.grid.get(x) == Some('@'))
                        .count()
                        < 4
                {
                    ret += 1;
                }
            }
        }

        ret
    }

    fn repeated_accessible_paper(&mut self) -> usize {
        let mut ret = 0;
        let mut q = VecDeque::default();

        for i in 0..self.grid.n {
            for j in 0..self.grid.m {
                let coord = (i, j).into();
                if self.grid[coord] == '@' {
                    q.push_back(coord);
                }
            }
        }

        while !q.is_empty() {
            let coord = q.pop_front().unwrap();

            if !self.grid.is_in_bounds(coord) {
                continue;
            }

            if self.grid[coord] == '.' {
                continue;
            }

            let count = coord
                .neighbours()
                .into_iter()
                .filter(|&x| self.grid.get(x) == Some('@'))
                .count();

            if count < 4 {
                for nb in coord.neighbours() {
                    if self.grid.get(nb) == Some('@') {
                        q.push_back(nb);
                    }
                }
                self.grid[coord] = '.';
                ret += 1;
            }
        }

        ret
    }
}

impl Problem for PrintingDepartment {
    const DAY: usize = 4;
    const TITLE: &'static str = "printing department";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.accessible_paper())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.repeated_accessible_paper())
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
        let solution = PrintingDepartment::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1491, 8722));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = PrintingDepartment::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(13, 43));
    }
}
