use common::read_file_to_string;
use common::Day;
pub mod day01;
pub mod day02;
pub mod day03;
use day02::Day02;
use day03::Day03;
use std::env;

fn main() {
    let mut take_input_mode: bool = false;
    let mut day: i64 = 0;
    for argument in env::args() {
        if take_input_mode {
            day = i64::from_str_radix(&argument, 10).unwrap();
        }
        if argument == "-d" {
            take_input_mode = true;
        }
    }

    match day {
        1 => {
            let contents: String = read_file_to_string("2022/day01_input.txt");
            let res1 = day01::part_one(&contents);
            println!("Part 1: {}", res1);
            let res2 = day01::part_two(&contents);
            println!("Part 2: {}", res2);
        }
        2 => {
            let mut day = Day02;
            day.run();
        }
        3 => {
            let mut day = Day03;
            day.run();
        }
        4 => {}
        5 => {
            // Day 5 depends on input from the user.
        }
        6 => {}
        _ => println!("No day selected, or day not yet implemented"),
    }
}
