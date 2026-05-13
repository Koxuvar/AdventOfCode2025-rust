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
        let width = lines[0].len();

        let boundaries: Vec<usize> = (0..width)
            .filter(|&c| lines.iter().all(|line| line.chars().nth(c) == Some(' ')))
            .collect();

        let mut delimiters = vec![0];
        delimiters.extend(boundaries.iter().map(|&b| b + 1));
        delimiters.push(width);

        let mut big_grid: Vec<Vec<Vec<char>>> = Vec::new();

        for i in 0..delimiters.len() - 1 {
            let start = delimiters[i];
            let end = delimiters[i + 1];
            let mut problem: Vec<Vec<char>> = Vec::new();
            for col in start..end {
                let column: Vec<char> = lines
                    .iter()
                    .map(|line| line.chars().nth(col).unwrap_or(' '))
                    .collect();
                problem.push(column);
            }
            big_grid.push(problem);
        }

        let mut total: u64 = 0;

        for mut problem in big_grid {
            if let Some(operator) = problem[0].pop() {
                let mut prob_total = 0;
                if operator == '*' {
                    prob_total += 1;
                };
                for row in problem {
                    if row.iter().collect::<String>().trim().is_empty() {
                        continue;
                    }
                    match operator {
                        '*' => {
                            let row_as_str: String = row.iter().collect();
                            prob_total *= row_as_str.trim().parse::<u64>().unwrap();
                        }
                        '+' => {
                            let row_as_str: String = row.iter().collect();
                            prob_total += row_as_str.trim().parse::<u64>().unwrap();
                        }
                        _ => println!("operator {} is an unmatched input", operator),
                    }
                }
                total += prob_total;
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {

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
