use crate::Solution;

pub struct Day3 {}

impl Solution for Day3 {
    fn part_one(&self, input: &str) -> String {
        let mut first = 0;
        let mut second = 0;
        let mut total = 0;

        for line in input.lines() {
            for ch in line.chars() {
                let digit = ch.to_digit(10).unwrap();
                if second > first {
                    first = second;
                    second = 0;
                };
                if first != 0 {
                    if digit > second {
                        second = digit;
                    };
                } else {
                    first = digit;
                }
            }

            total += (first * 10) + second;

            first = 0;
            second = 0;
        }

        format!("The total of all is {}", total)
    }

    fn part_two(&self, input: &str) -> String {
        let mut total = 0;

        for line in input.lines() {
            let mut removals = line.len() - 12;
            let mut stack: Vec<u32> = Vec::new();
            for ch in line.chars() {
                let digit = ch.to_digit(10).unwrap();
                while !stack.is_empty() && removals > 0 && *stack.last().unwrap() < digit {
                    stack.pop();
                    removals -= 1;
                }
                stack.push(digit);
            }

            stack.truncate(12);

            let mut num: u64 = 0;
            for d in stack {
                num = num * 10 + d as u64;
            }

            total += num;
        }

        format!("The total of all is {}", total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solution() -> Day3 {
        Day3 {}
    }

    // part one tests
    #[test]
    fn largest_at_end() {
        assert_eq!(
            solution().part_one("111111111111198"),
            "The total of all is 98"
        );
    }

    #[test]
    fn all_digits_identical() {
        assert_eq!(
            solution().part_one("333333333333333"),
            "The total of all is 33"
        );
    }

    #[test]
    fn largest_gap_between_answer() {
        assert_eq!(
            solution().part_one("211111111111119"),
            "The total of all is 29"
        );
    }

    #[test]
    fn smallest_possible_input() {
        assert_eq!(solution().part_one("23"), "The total of all is 23");
    }

    #[test]
    fn test_part_one() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solution().part_one(input), "The total of all is 357");
    }

    //part two tests
    #[test]
    fn exactly_12_digits() {
        assert_eq!(
            solution().part_two("123123123123"),
            "The total of all is 123123123123"
        );
    }

    #[test]
    fn large_at_end() {
        assert_eq!(
            solution().part_two("1111111111111987"),
            "The total of all is 111111111987"
        );
    }

    #[test]
    fn all_digits_identical_p2() {
        assert_eq!(
            solution().part_two("9999999999999999999"),
            "The total of all is 999999999999"
        );
    }

    #[test]
    fn test_part_two() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(
            solution().part_two(input),
            "The total of all is 3121910778619"
        );
    }
}
