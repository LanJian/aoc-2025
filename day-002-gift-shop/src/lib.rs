use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64,
    start_length: u32,
    end_length: u32,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self {
            start,
            end,
            start_length: start.ilog10() + 1,
            end_length: end.ilog10() + 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GiftShop {
    ranges: Vec<Range>,
}

impl FromStr for GiftShop {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = Vec::default();

        for token in s.trim().split(',') {
            let (s, e) = token
                .split_once('-')
                .ok_or_else(|| anyhow!("Could not parse range"))?;
            let start = s.parse()?;
            let end = e.parse().map(|x: u64| x + 1)?;
            ranges.push(Range::new(start, end));
        }

        Ok(Self { ranges })
    }
}

impl GiftShop {
    fn invalid_ids(&self) -> u64 {
        let mut ret = 0;

        for range in &self.ranges {
            let (s, e, n, m) = (range.start, range.end, range.start_length, range.end_length);
            let mut l = if n % 2 == 0 { n } else { n + 1 };

            'outer: while l <= m {
                let factor = 10u64.pow(l / 2);
                let left = (factor / 10).max(s / factor);

                for i in left..factor {
                    let candidate = i * factor + i;
                    if candidate >= e {
                        break 'outer;
                    }
                    if candidate >= s {
                        ret += candidate;
                    }
                }

                l += 2;
            }
        }

        ret

        //let mut ids = FxHashSet::default();

        //for interval in &self.intervals {
        //let (s, e) = (interval.start, interval.end);
        //let n = s.ilog10() + 1;
        //let m = e.ilog10() + 1;
        //let mut i = if n % 2 == 0 { n } else { n + 1 };

        //while i <= m {
        //Self::foo(interval, i, i / 2, &mut ids);
        //i += 2
        //}
        //}

        //ids.iter().sum()
    }

    fn more_invalid_ids(&self) -> u64 {
        let mut ids = FxHashSet::default();

        for range in &self.ranges {
            let (n, m) = (range.start_length, range.end_length);

            for i in n..=m {
                for l in 1..=i / 2 {
                    Self::gather_invalid_ids(range, i, l, &mut ids);
                }
            }
        }

        ids.iter().sum()
    }

    fn gather_invalid_ids(range: &Range, n: u32, l: u32, ids: &mut FxHashSet<u64>) {
        let (s, e) = (range.start, range.end);
        let temp = 10u64.pow(n);
        let floor = (temp / 10).max(s);
        let ceil = temp.min(e);
        let divisor = 10u64.pow(n - l);
        let mult = 10u64.pow(l);
        let a = floor / divisor;
        let b = (a + 1).min(mult - 1);

        let mut cur_a = a;
        let mut cur_b = b;
        let mut cur_gap = 1;
        let mut cur;

        loop {
            if cur_a >= ceil && cur_b >= ceil {
                return;
            } else if cur_a >= floor {
                cur = cur_a;
                break;
            } else if cur_b >= floor {
                cur = cur_b;
                break;
            }

            cur_a = cur_a * mult + a;
            cur_b = cur_b * mult + b;
            cur_gap = cur_gap * mult + 1;
        }

        while cur <= ceil {
            ids.insert(cur);
            cur += cur_gap;
        }
    }
}

impl Problem for GiftShop {
    const DAY: usize = 2;
    const TITLE: &'static str = "gift shop";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.invalid_ids())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.more_invalid_ids())
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
        let solution = GiftShop::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(19605500130, 36862281418));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let solution = GiftShop::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1227775554, 4174379265));
    }
}
