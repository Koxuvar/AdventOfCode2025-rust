use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn part_one(&self, input: &str) -> String {
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        let mut ingredients: Vec<u64> = Vec::new();
        let mut fresh: HashSet<u64> = HashSet::new();

        let split_input = input.split_at(input.find("\n\n").unwrap());

        for r in split_input.0.lines() {
            let rng: Vec<&str> = r.split("-").collect();
            ranges.push((
                rng[0].parse::<u64>().unwrap(),
                rng[1].parse::<u64>().unwrap(),
            ));
        }

        for t in split_input.1.lines() {
            if !t.is_empty() {
                ingredients.push(t.parse::<u64>().unwrap());
            }
        }

        for i in ingredients {
            if ranges.iter().any(|j| j.0 <= i && j.1 >= i) {
                fresh.insert(i);
            }
        }

        fresh.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut ranges: Vec<(u64, u64)> = Vec::new();

        let split_input = input.split_at(input.find("\n\n").unwrap());

        for r in split_input.0.lines() {
            let rng: Vec<&str> = r.split("-").collect();
            ranges.push((
                rng[0].parse::<u64>().unwrap(),
                rng[1].parse::<u64>().unwrap(),
            ));
        }

        ranges.sort_by_key(|r| r.0);

        let mut merged: Vec<(u64, u64)> = Vec::new();
        for (start, end) in ranges {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 + 1 {
                    last.1 = last.1.max(end);
                    continue;
                }
            }
            merged.push((start, end));
        }
        // Brute Force Approace - takes forever
        // for r in ranges {
        //     for i in r.0..=r.1 {
        //         fresh.insert(i);
        //     }
        // }
        let total: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();

        total.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn solution() -> Day5 {
        Day5 {}
    }

    #[test]
    fn aoc_input_part_one() {
        assert_eq!(
            solution().part_one(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            "3"
        );
    }

    #[test]
    fn aoc_input_part_two() {
        assert_eq!(
            solution().part_two(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            "14"
        );
    }
}
