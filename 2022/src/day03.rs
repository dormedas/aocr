use common::read_file_to_string;
use common::Day;
use std::collections::HashSet;

pub struct Day03;

const LOWER_OFFSET: usize = 96;
const UPPER_OFFSET: usize = 38;

impl Day03 {
    fn calculate_value(&self, character: char) -> usize {
        if character.is_uppercase() {
            character as usize - UPPER_OFFSET
        } else {
            character as usize - LOWER_OFFSET
        }
    }
}

impl Day for Day03 {
    fn run(&mut self) {
        let input = read_file_to_string("2022/day03_input.txt");
        let mut total = 0;

        for line in input.lines() {
            let (comp_one, comp_two) = line.split_at(line.len() / 2);
            let comp_one_set = comp_one.chars().collect::<HashSet<char>>();
            let comp_two_set = comp_two.chars().collect::<HashSet<char>>();

            let intersection = comp_one_set.intersection(&comp_two_set);
            let val: char = **intersection.collect::<Vec<&char>>().get(0).unwrap();
            total += self.calculate_value(val);
        }

        println!("Part One: {}", total);

        total = 0;

        for chunks in input.lines().collect::<Vec<&str>>().chunks(3) {
            let chunk_vec = chunks.iter().collect::<Vec<&&str>>();

            let elf_one_set = chunk_vec.get(0).unwrap().chars().collect::<HashSet<char>>();
            let elf_two_set = chunk_vec.get(1).unwrap().chars().collect::<HashSet<char>>();
            let elf_three_set = chunk_vec.get(2).unwrap().chars().collect::<HashSet<char>>();

            let intersected_set = elf_one_set
                .intersection(&elf_two_set)
                .map(|ref_char| *ref_char)
                .collect::<HashSet<char>>();

            let intersection = intersected_set.intersection(&elf_three_set);
            let val: char = **intersection.collect::<Vec<&char>>().get(0).unwrap();

            total += self.calculate_value(val);
        }

        println!("Part Two: {}", total);
    }
}
