use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.as_bytes().try_into().map(Node)?)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct Memo {
    dac_count: usize,
    fft_count: usize,
    both_count: usize,
    none_count: usize,
}

#[derive(Debug, Clone)]
pub struct Reactor {
    adj: FxHashMap<Node, FxHashSet<Node>>,
}

impl FromStr for Reactor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut adj = FxHashMap::default();

        for line in s.lines() {
            let (left, right) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("Invalid input line"))?;

            let node = Node::from_str(left)?;
            let ns: FxHashSet<Node> = right
                .split_whitespace()
                .map(Node::from_str)
                .collect::<Result<_, _>>()?;
            adj.insert(node, ns);
        }

        Ok(Self { adj })
    }
}

impl Reactor {
    fn dfs(&self, node: &Node, memo: &mut FxHashMap<Node, usize>) -> usize {
        if let Some(&count) = memo.get(node) {
            return count;
        }

        if *node == Node(*b"out") {
            return 1;
        }

        let ret = self.adj[node].iter().map(|x| self.dfs(x, memo)).sum();
        memo.insert(*node, ret);
        ret
    }

    fn dfs_with_constraints(&self, node: &Node, memo: &mut FxHashMap<Node, Memo>) -> Memo {
        if let Some(&m) = memo.get(node) {
            return m;
        }

        if *node == Node(*b"out") {
            return Memo {
                dac_count: 0,
                fft_count: 0,
                both_count: 0,
                none_count: 1,
            };
        }

        let mut ret = Memo::default();
        for n in &self.adj[node] {
            let result = self.dfs_with_constraints(n, memo);
            ret.dac_count += result.dac_count;
            ret.fft_count += result.fft_count;
            ret.both_count += result.both_count;
            ret.none_count += result.none_count;

            match &node.0 {
                b"dac" => {
                    ret.dac_count += result.none_count;
                    ret.both_count += result.fft_count;
                }
                b"fft" => {
                    ret.fft_count += result.none_count;
                    ret.both_count += result.dac_count;
                }
                _ => (),
            };
        }

        memo.insert(*node, ret);
        ret
    }
}

impl Problem for Reactor {
    const DAY: usize = 11;
    const TITLE: &'static str = "reactor";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.dfs(&Node(*b"you"), &mut FxHashMap::default()))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let result = self.dfs_with_constraints(&Node(*b"svr"), &mut FxHashMap::default());
        Ok(result.both_count)
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
        let solution = Reactor::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(670, 332052564714990));
    }

    #[test]
    fn example_part_one() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let mut problem = Reactor::from_str(&input).unwrap();
        assert_eq!(problem.part_one().unwrap(), 5);
    }

    #[test]
    fn example_part_two() {
        let input = std::fs::read_to_string("example2.txt").expect("Unable to load input");
        let mut problem = Reactor::from_str(&input).unwrap();
        assert_eq!(problem.part_two().unwrap(), 2);
    }
}
