use std::str::FromStr;

use anyhow::{Result, anyhow};
use aoc_common::grid::Coordinate;
use aoc_plumbing::Problem;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, Default)]
struct BitSet {
    bitset: (u128, u128),
}

impl FromStr for BitSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut bitset = Self { bitset: (0, 0) };
        for (i, c) in s.chars().enumerate() {
            if c != '^' {
                continue;
            }

            bitset.set(i);
        }
        Ok(bitset)
    }
}

impl BitSet {
    fn set(&mut self, i: usize) {
        if i < 128 {
            let mask = 1 << (127 - i);
            self.bitset.0 |= mask;
        } else {
            let mask = 1 << (255 - i);
            self.bitset.1 |= mask;
        }
    }

    fn _unset(&mut self, i: usize) {
        if i < 128 {
            let mask = 1 << (127 - i);
            self.bitset.0 &= !mask;
        } else {
            let mask = 1 << (255 - i);
            self.bitset.1 &= !mask;
        }
    }

    fn is_zero(&self) -> bool {
        self.bitset.0 == 0 && self.bitset.1 == 0
    }

    fn bitand(&self, rhs: &Self) -> Self {
        Self {
            bitset: (self.bitset.0 & rhs.bitset.0, self.bitset.1 & rhs.bitset.1),
        }
    }

    fn bitor(&self, rhs: &Self) -> Self {
        Self {
            bitset: (self.bitset.0 | rhs.bitset.0, self.bitset.1 | rhs.bitset.1),
        }
    }

    fn bitxor(&self, rhs: &Self) -> Self {
        Self {
            bitset: (self.bitset.0 ^ rhs.bitset.0, self.bitset.1 ^ rhs.bitset.1),
        }
    }

    fn bitshift_left(&self, rhs: usize) -> Self {
        if rhs == 0 {
            return *self;
        }

        if rhs >= 256 {
            return Self::default();
        }

        let mut ret = Self::default();

        if rhs <= 128 {
            ret.bitset.0 = self.bitset.0 << rhs | self.bitset.1 >> (128 - rhs);
            ret.bitset.1 = self.bitset.1 << rhs;
        } else {
            ret.bitset.0 = self.bitset.1 << (rhs - 128);
        }

        ret
    }

    fn bitshift_right(&self, rhs: usize) -> Self {
        if rhs == 0 {
            return *self;
        }

        if rhs >= 256 {
            return Self::default();
        }

        let mut ret = Self::default();

        if rhs <= 128 {
            ret.bitset.0 = self.bitset.0 >> rhs;
            ret.bitset.1 = self.bitset.1 >> rhs | self.bitset.0 << (128 - rhs);
        } else {
            ret.bitset.1 = self.bitset.0 >> (rhs - 128);
        }

        ret
    }

    fn count_ones(&self) -> usize {
        (self.bitset.0.count_ones() + self.bitset.1.count_ones()) as usize
    }

    fn leading_zeros(&self) -> usize {
        if self.bitset.0 == 0 {
            self.bitset.1.leading_zeros() as usize + 128
        } else {
            self.bitset.0.leading_zeros() as usize
        }
    }
}

#[derive(Debug, Clone)]
pub struct Laboratories {
    start: Coordinate,
    splitters: Vec<BitSet>,
}

impl FromStr for Laboratories {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut splitters = Vec::default();

        for (i, l) in s.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                if c == 'S' {
                    start = Some(Coordinate::from((i, j)));
                }
            }
            let bitset = BitSet::from_str(l)?;
            if !bitset.is_zero() {
                splitters.push(bitset);
            }
        }

        Ok(Self {
            start: start.ok_or_else(|| anyhow!("Could not find start"))?,
            splitters,
        })
    }
}

impl Laboratories {
    fn simulate(&self) -> usize {
        let mut ret = 0;
        let mut cur = BitSet::default();
        cur.set(self.start.col() as usize);

        for splitter_array in &self.splitters {
            let overlap = cur.bitand(splitter_array);
            let splitted = overlap.bitshift_left(1).bitor(&overlap.bitshift_right(1));
            cur = cur.bitxor(&overlap).bitor(&splitted);
            ret += overlap.count_ones();
        }

        ret
    }

    fn simulate_quantum(
        &self,
        row: usize,
        col: usize,
        memo: &mut FxHashMap<(usize, usize), usize>,
    ) -> usize {
        if row >= self.splitters.len() {
            return 1;
        }

        if let Some(&x) = memo.get(&(row, col)) {
            return x;
        }

        let mut ret = 0;
        let mut cur = BitSet::default();
        cur.set(col);

        for i in row..self.splitters.len() {
            let splitter_array = self.splitters[i];
            let overlap = cur.bitand(&splitter_array);
            if overlap.is_zero() {
                continue;
            }

            let position = overlap.leading_zeros();
            if position > 0 {
                ret += self.simulate_quantum(i, position - 1, memo);
            }
            ret += self.simulate_quantum(i, position + 1, memo);

            memo.insert((row, col), ret);
            return ret;
        }

        memo.insert((row, col), 1);
        1
    }
}

impl Problem for Laboratories {
    const DAY: usize = 7;
    const TITLE: &'static str = "laboratories";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.simulate())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.simulate_quantum(0, self.start.col() as usize, &mut FxHashMap::default()))
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
        let solution = Laboratories::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1516, 1393669447690));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = Laboratories::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(21, 40));
    }
}
