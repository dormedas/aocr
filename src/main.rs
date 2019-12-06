fn main() {
    //days::one::run();

    //2-1
    days::two::run(12, 2);

    //2-2
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
}