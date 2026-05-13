use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day7 {}

fn laser_beam_pt_one(
    input_grid: &Vec<Vec<char>>,
    start_idx: usize,
    start_row: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> u64 {
    let mut count = 0;
    for row in start_row..input_grid.len() {
        match input_grid[row][start_idx] {
            '.' => continue,
            '^' => {
                if visited.contains(&(row, start_idx)) {
                    break;
                }
                visited.insert((row, start_idx));
                count += 1;
                if start_idx + 1 < input_grid[0].len() {
                    count += laser_beam_pt_one(input_grid, start_idx + 1_usize, row, visited);
                }
                if start_idx > 0 {
                    count += laser_beam_pt_one(input_grid, start_idx - 1, row, visited);
                }
                break;
            }
            _ => continue,
        }
    }

    count
}

fn laser_beam_pt_two(
    input_grid: &Vec<Vec<char>>,
    start_idx: usize,
    start_row: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    for row in start_row..input_grid.len() {
        if input_grid[row][start_idx] == '^' {
            if let Some(&cached) = memo.get(&(row, start_idx)) {
                return cached;
            }
            let left = if start_idx > 0 {
                laser_beam_pt_two(input_grid, start_idx - 1, row, memo)
            } else {
                0
            };
            let right = if start_idx + 1 < input_grid[0].len() {
                laser_beam_pt_two(input_grid, start_idx + 1, row, memo)
            } else {
                0
            };
            let result = left + right;
            memo.insert((row, start_idx), result);
            return result;
        }
    }
    1
}

impl Solution for Day7 {
    fn part_one(&self, input: &str) -> String {
        let mut input_grid: Vec<Vec<char>> = Vec::new();
        for line in input.lines().collect::<Vec<&str>>() {
            input_grid.push(line.chars().collect());
        }

        let start_index = input_grid[0].iter().position(|x| x == &'S').unwrap();
        let mut splitter_idxs: HashSet<(usize, usize)> = HashSet::new();

        let total_splits = laser_beam_pt_one(&input_grid, start_index, 1_usize, &mut splitter_idxs);

        total_splits.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut input_grid: Vec<Vec<char>> = Vec::new();
        for line in input.lines().collect::<Vec<&str>>() {
            input_grid.push(line.chars().collect());
        }

        let start_index = input_grid[0].iter().position(|x| x == &'S').unwrap();
        let mut splitter_idxs: HashMap<(usize, usize), u64> = HashMap::new();

        let total_splits = laser_beam_pt_two(&input_grid, start_index, 1_usize, &mut splitter_idxs);

        total_splits.to_string()
    }
}

// .......S.......
// ...............
// .......^.......
// ...............
// ......^.^......
// ...............
// .....^.^.^.....
// ...............
// ....^.^...^....
// ...............
// ...^.^...^.^...
// ...............
// ..^...^.....^..
// ...............
// .^.^.^.^.^...^.
// ...............
