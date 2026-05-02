use crate::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn part_one(&self, input: &str) -> String {
        let ids: Vec<&str> = input.split(",").collect();
        let mut total = 0;

        for id_grp in ids {
            let id_range = match id_grp.split_once("-") {
                Some(d) => d,
                None => continue,
            };

            let mut low_str = id_range.0.trim();
            let high_str = id_range.1.trim();

            let mut low_as_int = low_str.parse::<u64>().unwrap_or_else(|err| {
                println!("Issue turning low value into int, {}", err);
                0
            });
            let high_as_int = high_str.parse::<u64>().unwrap_or_else(|err| {
                println!("Issue turning high value into int, {}", err);
                0
            });

            if low_str.len() < 2 {
                low_as_int = 10;
                low_str = "10";
            };

            let low_half_digits = 10u64.pow((low_str.len() as u32 / 2) - 1);
            let high_half_digits = 10u64.pow(high_str.len() as u32 / 2) - 1;

            for i in low_half_digits..=high_half_digits {
                let num_to_check = (i * 10u64.pow(i.ilog10() + 1)) + i;
                if num_to_check > low_as_int && num_to_check < high_as_int {
                    total += num_to_check;
                };
            }
        }

        format!("The total is {}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let ids: Vec<&str> = input.split(",").collect();
        let mut total = 0;

        for id_grp in ids {
            let id_range = match id_grp.split_once("-") {
                Some(d) => d,
                None => continue,
            };

            let low_str = id_range.0.trim();
            let high_str = id_range.1.trim();

            let low_as_int = low_str.parse::<u64>().unwrap_or_else(|err| {
                println!("Issue turning low value into int, {}", err);
                0
            });
            let high_as_int = high_str.parse::<u64>().unwrap_or_else(|err| {
                println!("Issue turning high value into int, {}", err);
                0
            });

            for i in low_as_int..=high_as_int {
                if is_repeat(i.to_string().as_str()) {
                    total += i;
                }
            }
        }

        format!("The total is {}", total)
    }
}

fn is_repeat(s: &str) -> bool {
    let n = s.len();
    for k in 1..n {
        if n.is_multiple_of(k) {
            let unit = &s[..k];
            if unit.repeat(n / k) == s {
                return true;
            }
        }
    }
    false
}
