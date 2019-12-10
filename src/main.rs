use days::Problem;
use days::day01;
use days::day02;
use days::day03::*;
use days::day04;
use days::day05;
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
            day01::part_one();
            day01::part_two();
        },
        2 => {
            day02::part_one();
            day02::part_two();
        },
        3 => {
            println!("Day 3 takes a long while in debug...");
            let mut day03: Day03 = Day03 {
                wire_head: (0, 0, 0)
            };
        
            let input: &str = "input";
        
            println!("Part one:");
            day03.part_one(input);
            println!("Part two:");
            day03.wire_head = (0, 0, 0);
            day03.part_two(input);
        },
        4 => {
            println!("{}", day04::part_one());
            println!("{}", day04::part_two());
        },
        5 => {
            // Day 5 depends on input from the user.
            println!("Day 5 depends on input from the user. When prompted, enter 1 for part one, and 5 for part 2");
            day05::part_one();
        },
        _ => println!("No day selected, or day not yet implemented"),
    }
}