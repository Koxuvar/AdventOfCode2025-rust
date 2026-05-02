use aoc2025::{Solution, days::*};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: String,
}

fn main() {
    let day = Args::parse().day;

    let input_file_path = format!("./inputs/day{}.txt", day);

    let file = match std::fs::read_to_string(input_file_path) {
        Ok(f) => f,
        Err(_) => return,
    };

    match day.as_str() {
        "1" => {
            println!("Day {} solutions:", day);
            println!("Part one:{}", Day1 {}.part_one(&file));
            println!("Part two:{}", Day1 {}.part_two(&file));
        }
        "2" => {
            println!("Day {} solutions:", day);
            println!("{}", Day2 {}.part_one(&file));
            println!("{}", Day2 {}.part_two(&file));
        }
        "3" => {
            println!("Day {} solutions:", day);
            println!("{}", Day3 {}.part_one(&file));
            println!("{}", Day3 {}.part_two(&file));
        }
        "4" => {
            println!("Day {} solutions:", day);
            println!("{}", Day4 {}.part_one(&file));
            println!("{}", Day4 {}.part_two(&file));
        }
        "5" => {
            println!("Day {} solutions:", day);
            println!("{}", Day5 {}.part_one(&file));
            println!("{}", Day5 {}.part_two(&file));
        }
        "6" => {
            println!("Day {} solutions:", day);
            println!("{}", Day6 {}.part_one(&file));
            println!("{}", Day6 {}.part_two(&file));
        }
        "7" => {
            println!("Day {} solutions:", day);
            println!("{}", Day7 {}.part_one(&file));
            println!("{}", Day7 {}.part_two(&file));
        }
        "8" => {
            println!("Day {} solutions:", day);
            println!("{}", Day8 {}.part_one(&file));
            println!("{}", Day8 {}.part_two(&file));
        }
        "9" => {
            println!("Day {} solutions:", day);
            println!("{}", Day9 {}.part_one(&file));
            println!("{}", Day9 {}.part_two(&file));
        }
        "10" => {
            println!("Day {} solutions:", day);
            println!("{}", Day10 {}.part_one(&file));
            println!("{}", Day10 {}.part_two(&file));
        }
        "11" => {
            println!("Day {} solutions:", day);
            println!("{}", Day11 {}.part_one(&file));
            println!("{}", Day11 {}.part_two(&file));
        }
        "12" => {
            println!("Day {} solutions:", day);
            println!("{}", Day12 {}.part_one(&file));
            println!("{}", Day12 {}.part_two(&file));
        }
        _ => {}
    }
}
