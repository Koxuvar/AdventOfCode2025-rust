use crate::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn part_one(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut grid: Vec<Vec<&str>> = Vec::new();
        let mut total = 0;

        for line in lines {
            grid.push(line.split_whitespace().collect());
        }

        let width = grid[0].len();
        let height = grid.len();

        let mut counter = 0;

        while counter < width {
            let mut buf = 0;
            match grid[height - 1][counter] {
                "+" => {
                    for row in grid.iter().take(height - 1) {
                        buf += row[counter].parse::<u64>().unwrap();
                    }
                }
                "*" => {
                    buf = 1;
                    for row in grid.iter().take(height - 1) {
                        buf *= row[counter].parse::<u64>().unwrap();
                    }
                }
                _ => continue,
            }
            total += buf;
            counter += 1;
        }

        total.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let big_grid: Vec<Vec<Vec<char>>> = Vec::new();
        let mut grid: Vec<Vec<&str>> = Vec::new();
        let mut total = 0;

        for line in lines {
            grid.push(line.split_whitespace().collect());
        }

        let width = grid[0].len();
        let height = grid.len();

        let mut counter = 0;

        while counter < width {
            let mut buf = 0;
            match grid[height - 1][counter] {
                "+" => {
                    for row in grid.iter().take(height - 1) {
                        buf += row[counter].parse::<u64>().unwrap();
                    }
                }
                "*" => {
                    buf = 1;
                    for row in grid.iter().take(height - 1) {
                        buf *= row[counter].parse::<u64>().unwrap();
                    }
                }
                _ => continue,
            }
            total += buf;
            counter += 1;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solution() -> Day6 {
        Day6 {}
    }

    #[test]
    fn aoc_test_input_part_one() {
        assert_eq!(
            "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
            "4277556"
        );
    }

    #[test]
    fn aoc_test_input_part_two() {
        assert_eq!(
            "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ",
            "3263827"
        );
    }
}
