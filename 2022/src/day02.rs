use common::read_file_to_string;
use common::Day;

pub struct Day02;

const VALUE_TABLE: [[usize; 3]; 3] = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];

const CHOICE_TABLE: [[usize; 3]; 3] = [[2, 0, 1], [0, 1, 2], [1, 2, 0]];

impl Day02 {
    fn run_game_p1(their_shape: char, our_shape: char) -> usize {
        let their_index = match their_shape {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!(),
        };

        let our_index = match our_shape {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!(),
        };

        VALUE_TABLE[their_index][our_index]
    }

    fn run_game_p2(their_shape: char, our_goal: char) -> usize {
        let their_index = match their_shape {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => panic!(),
        };

        let our_index = match our_goal {
            'X' => 0,
            'Y' => 1,
            'Z' => 2,
            _ => panic!(),
        };

        VALUE_TABLE[their_index][CHOICE_TABLE[our_index][their_index]]
    }
}

impl Day for Day02 {
    fn run(&mut self) {
        let file = read_file_to_string("2022/day02_input.txt");

        let mut p1_total = 0;
        let mut p2_total = 0;

        for line in file.lines() {
            let theirs = line.chars().nth(0).unwrap();
            let ours = line.chars().nth(2).unwrap();
            p1_total += Day02::run_game_p1(theirs, ours);
            p2_total += Day02::run_game_p2(theirs, ours);
        }

        println!("Part 1 Score: {}", p1_total);
        println!("Part 2 Score: {}", p2_total);
    }
}
