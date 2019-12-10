use days::Problem;

fn main() {
    //days::one::run();

    //2-1
    //days::two::run(12, 2);

    //2-2
    /*
    let mut found_answer: bool = false;
    for i in 0..99 {
        for j in 0..99 {
            let result: i64 = days::two::run(i, j);
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
    */

    let mut day03:days::Day03 = days::Day03{
        wire_head: (0, 0, 0)
    };

    let input: &str = "input";

    println!("Part one:");
    //day03.part_one(input);
    println!("Part two:");
    day03.part_two(input);
}