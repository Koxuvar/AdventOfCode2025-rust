use crate::Solution;

pub struct Day4 {}

impl Solution for Day4 {
    fn part_one(&self, input: &str) -> String {
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut col: Vec<char> = Vec::new();
        let mut total_rolls: i64 = 0;

        for line in input.lines() {
            for ch in line.chars() {
                col.push(ch);
            }
            rows.push(col.clone());
            col.clear();
        }

        for (i, row) in rows.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == '.' {
                    continue;
                };

                let mut surrounding_count = 0;

                for dr in -1_i32..=1 {
                    for dc in -1_i32..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = i as i32 + dr;
                        let nc = j as i32 + dc;
                        if nr < 0 || nc < 0 || nr >= rows.len() as i32 || nc >= rows[0].len() as i32
                        {
                            continue;
                        }
                        if rows[nr as usize][nc as usize] == '@' {
                            surrounding_count += 1;
                        }
                    }
                }
                if surrounding_count < 4 {
                    total_rolls += 1;
                }
            }
        }

        total_rolls.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut rows: Vec<Vec<char>> = Vec::new();
        let mut col: Vec<char> = Vec::new();
        let mut total_rolls: i64 = 0;

        for line in input.lines() {
            for ch in line.chars() {
                col.push(ch);
            }
            rows.push(col.clone());
            col.clear();
        }

        loop {
            let mut changed = false;
            let mut to_remove: Vec<(usize, usize)> = Vec::new();
            for (i, row) in rows.iter().enumerate() {
                for (j, col) in row.iter().enumerate() {
                    if *col == '.' {
                        continue;
                    };

                    let mut surrounding_count = 0;

                    for dr in -1_i32..=1 {
                        for dc in -1_i32..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = i as i32 + dr;
                            let nc = j as i32 + dc;
                            if nr < 0
                                || nc < 0
                                || nr >= rows.len() as i32
                                || nc >= rows[0].len() as i32
                            {
                                continue;
                            }
                            if rows[nr as usize][nc as usize] == '@' {
                                surrounding_count += 1;
                            }
                        }
                    }

                    if rows[i][j] == '@' && surrounding_count < 4 {
                        to_remove.push((i, j));
                    }
                }
            }
            for (i, j) in to_remove {
                rows[i][j] = '.';
                total_rolls += 1;
                changed = true;
            }
            if !changed {
                break;
            }
        }

        total_rolls.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solution() -> Day4 {
        Day4 {}
    }

    #[test]
    fn aoc_input() {
        assert_eq!(
            solution().part_one(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            "13"
        );
    }

    #[test]
    fn single_center() {
        assert_eq!(
            solution().part_one(
                "...
.@.
..."
            ),
            "1"
        );
    }

    #[test]
    fn no_roll() {
        assert_eq!(
            solution().part_one(
                "...
...
..."
            ),
            "0"
        );
    }

    #[test]
    fn oops_all_rolls() {
        assert_eq!(
            solution().part_one(
                "@@@
@@@
@@@"
            ),
            "4"
        );
    }

    #[test]
    fn aoc_input_part_two() {
        assert_eq!(
            solution().part_two(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            "43"
        );
    }
}
