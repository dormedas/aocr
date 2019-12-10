use super::intcode;

pub fn part_one() {
    intcode::run("two_input.txt", 12, 2);
}

pub fn part_two() {
    let mut found_answer: bool = false;
    for i in 0..99 {
        for j in 0..99 {
            let result: i64 = intcode::run("two_input.txt", i, j);
            if result == 19690720 {
                println!("noun: {} verb: {}", i, j);
                println!("Answer: {}", 100 * i + j);
                found_answer = true;
                break;
            }
        }
        if found_answer {
            break;
        }
    }
}