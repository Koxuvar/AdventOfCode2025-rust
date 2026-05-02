use crate::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn part_one(&self, input: &str) -> String {
        let mut position = 50;
        let mut counter = 0;
        let spins: Vec<&str> = input.lines().collect();

        for spin in spins {
            let split_spin = spin.split_at(1);
            match split_spin.0 {
                "L" => {
                    position = wrap_to_99(position + split_spin.1.parse::<i32>().unwrap());
                    position
                } //add nums
                "R" => {
                    position = wrap_to_99(position - split_spin.1.parse::<i32>().unwrap());
                    position
                } //subtract nums
                _ => position,
            };
            if position == 0 {
                counter += 1;
            }
        }

        counter.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut position = (50, 0);
        let mut counter = 0;
        let spins: Vec<&str> = input.lines().collect();

        for spin in spins {
            let split_spin = spin.split_at(1);
            match split_spin.0 {
                "L" => {
                    position = wrap_to_99_check_spin(
                        position.0,
                        position.0 - split_spin.1.parse::<i32>().unwrap(),
                    );
                } //add nums
                "R" => {
                    position = wrap_to_99_check_spin(
                        position.0,
                        position.0 + split_spin.1.parse::<i32>().unwrap(),
                    );
                } //subtract nums
                _ => (),
            };
            if position.1 != 0 {
                counter += position.1;
            }
        }

        counter.to_string()
    }
}

fn wrap_to_99(input: i32) -> i32 {
    let max = 99;
    let modulus = max + 1;
    ((input % modulus) + modulus) % modulus
}

//my first pass at the solution for part 2
#[allow(dead_code)]
fn wrap_to_99_check_spin_old(prev_position: i32, input: i32) -> (i32, i32) {
    let mut wraps_or_0 = 0;
    if !(0..=99).contains(&input) {
        if input < 0 {
            wraps_or_0 = 1 + input.abs() / 100;
        } else {
            wraps_or_0 = input / 100;
        };
        if prev_position == 0 {
            wraps_or_0 -= 1;
        }
    };
    if input == 0 && prev_position != 0 {
        wraps_or_0 += 1;
    };
    let max = 99;
    let modulus = max + 1;
    (((input % modulus) + modulus) % modulus, wraps_or_0)
}

//adapted solution after looking at reddit answer
fn wrap_to_99_check_spin(prev_position: i32, input: i32) -> (i32, i32) {
    let mut crossings = (input.div_euclid(100) - prev_position.div_euclid(100)).abs();

    if input < prev_position {
        if prev_position == 0 {
            crossings -= 1
        };
        if input.rem_euclid(100) == 0 {
            crossings += 1
        };
    };

    (input.rem_euclid(100), crossings)
}

//found off of reddit - correct solution in one function
//
// fn part_two(&self, input: &str) -> String {
//     let mut current: i32 = 50;
//     let mut visited_zeroes = 0;
//
//     for line in input.lines() {
//         let direction = line.chars().next().unwrap();
//         let mut value: i32 = line[1..].parse().unwrap();
//         if direction == 'L' {
//             value *= -1;
//         }
//
//         let previous = current;
//         current += value;
//
//         let mut crossings = (current.div_euclid(100) - previous.div_euclid(100)).abs();
//         if direction == 'L' {
//             if previous == 0 {
//                 crossings -= 1
//             }
//             if current.rem_euclid(100) == 0 {
//                 crossings += 1;
//             }
//         }
//         visited_zeroes += crossings;
//
//         current = current.rem_euclid(100);
//     }
//     println!("Final position: {}", current);
//     println!("Number of visited 0s: {}", visited_zeroes);
//     visited_zeroes.to_string()
// }
