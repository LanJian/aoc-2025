use std::{cmp, collections::BinaryHeap, str::FromStr};

use aoc_common::algebra::Point3;
use aoc_plumbing::Problem;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
struct PointPair {
    a: Point3<u64>,
    b: Point3<u64>,
    dist_sq: u64,
}

impl PointPair {
    pub fn new(a: Point3<u64>, b: Point3<u64>) -> Self {
        let dx = a.x.abs_diff(b.x);
        let dy = a.y.abs_diff(b.y);
        let dz = a.z.abs_diff(b.z);

        Self {
            a,
            b,
            dist_sq: dx * dx + dy * dy + dz * dz,
        }
    }
}

impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PointPair {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.dist_sq.cmp(&self.dist_sq)
    }
}

#[derive(Debug, Clone)]
pub struct Playground {
    points: Vec<Point3<u64>>,
    point_pairs: BinaryHeap<PointPair>,
    example: usize,
    part1: usize,
    part2: u64,
}

impl FromStr for Playground {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::default();
        let mut point_pairs = BinaryHeap::new();

        for l in s.lines() {
            let parsed: Vec<_> = l
                .split(',')
                .map(|x| x.parse::<u64>())
                .collect::<Result<_, _>>()?;
            let point = Point3::new(parsed[0], parsed[1], parsed[2]);

            for p in &points {
                point_pairs.push(PointPair::new(*p, point));
            }

            points.push(point);
        }

        let mut ret = Self {
            points,
            point_pairs,
            example: 0,
            part1: 0,
            part2: 0,
        };

        ret.process();

        Ok(ret)
    }
}

impl Playground {
    fn root(
        point: &Point3<u64>,
        adj: &FxHashMap<Point3<u64>, FxHashSet<Point3<u64>>>,
    ) -> Point3<u64> {
        let ns = &adj[point];
        if ns.len() == 1 {
            // ok to unwrap, we know len is 1
            return *ns.iter().next().unwrap();
        }
        *point
    }

    fn process(&mut self) {
        let mut adj: FxHashMap<Point3<u64>, FxHashSet<Point3<u64>>> = self
            .points
            .iter()
            .map(|x| (*x, FxHashSet::default()))
            .collect();

        let mut i = 0;
        while let Some(pair) = self.point_pairs.pop() {
            i += 1;
            let a_root = Self::root(&pair.a, &adj);
            let b_root = Self::root(&pair.b, &adj);

            if a_root != b_root {
                // move the smaller group to bigger group
                let mut small_group = FxHashSet::default();
                let big_group_root = if adj[&a_root].len() < adj[&b_root].len() {
                    small_group.extend(&adj[&a_root]);
                    small_group.insert(a_root);
                    b_root
                } else {
                    small_group.extend(&adj[&b_root]);
                    small_group.insert(b_root);
                    a_root
                };

                for p in &small_group {
                    adj.entry(*p).and_modify(|x| {
                        x.clear();
                        x.insert(big_group_root);
                    });
                    adj.entry(big_group_root).and_modify(|x| {
                        x.insert(*p);
                    });
                }

                if adj[&big_group_root].len() == self.points.len() - 1 {
                    self.part2 = pair.a.x * pair.b.x;

                    if self.part1 > 0 {
                        return;
                    }
                }
            }

            if i == 10 {
                let mut circuits: Vec<_> = adj.values().map(|v| v.len() + 1).collect();
                circuits.sort();
                self.example = circuits.iter().rev().take(3).product();
            } else if i == 1000 {
                let mut circuits: Vec<_> = adj.values().map(|v| v.len() + 1).collect();
                circuits.sort();
                self.part1 = circuits.iter().rev().take(3).product();

                if self.part2 > 0 {
                    return;
                }
            }
        }
    }
}

impl Problem for Playground {
    const DAY: usize = 8;
    const TITLE: &'static str = "playground";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part2)
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
        let solution = Playground::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(97384, 9003685096));
    }

    #[test]
    fn example() {
        let input = std::fs::read_to_string("example.txt").expect("Unable to load input");
        let problem = Playground::from_str(&input).unwrap();
        assert_eq!(problem.example, 40);
        assert_eq!(problem.part2, 25272);
    }
}
