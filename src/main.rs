use std::io;
use std::string;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("one_input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents);

    let masses_str: Vec<&str> = contents.split('\n').collect();

    let mut masses: Vec<i64> = Vec::new();

    for i in &masses_str {
        if i.is_empty() {
            continue;
        }
        let converted: i64 = i64::from_str_radix(i, 10).unwrap();
        masses.push(converted);
    }


    let mut fuel_requirement_total: i64 = 0;
    for i in &masses {
        fuel_requirement_total += days::one::generate_fuel_for_module(*i);
    }

    println!("{}", fuel_requirement_total);
}