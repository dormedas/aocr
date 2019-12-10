pub fn load_program(program: &String, memory: &mut Vec<i64>) {
    let program_strs: Vec<&str> = program.split(',').collect();

    memory.clear();

    for i in &program_strs {
        //println!("{}", i);
        let converted: i64 = i64::from_str_radix(i.trim(), 10).unwrap();
        memory.push(converted);
    }
}

// Runs program that is loaded into the passed in memory, 
pub fn run_program(memory: &mut Vec<i64>) -> String {
    let mut i: usize = 0;
    while i < memory.len() {
        let opcode: i64 = memory[i];
        match opcode {
            1 => {
                let output_index: usize = memory[i+3] as usize;
                let input_one_index: usize = memory[i+1] as usize;
                let input_two_index: usize = memory[i+2] as usize;
                memory[output_index] = memory[input_one_index] + memory[input_two_index];
            },
            2 => {
                let output_index: usize = memory[i+3] as usize;
                let input_one_index: usize = memory[i+1] as usize;
                let input_two_index: usize = memory[i+2] as usize;
                memory[output_index] = memory[input_one_index] * memory[input_two_index];
            },
            99 => {
                break;
            },
            _ => panic!()
        }
        i += 4;
    }

    let mut output: String = String::new();
    
    for j in memory {
        output.push_str(&(j.to_string() + ","));
    }

    // Remove the trailing comma
    output.remove(output.len() - 1);

    //println!("{}", output);

    output
}

pub fn part_one() {
    run(12, 2);
}

pub fn part_two() {
    let mut found_answer: bool = false;
    for i in 0..99 {
        for j in 0..99 {
            let result: i64 = run(i, j);
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

pub fn run(noun: i64, verb: i64) -> i64 {
    let mut memory: Vec<i64> = Vec::new();

    let contents: String = super::common::read_file_to_string("two_input.txt");

    load_program(&contents, &mut memory);

    memory[1] = noun;
    memory[2] = verb;

    run_program(&mut memory);

    memory[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_intcode_computer() {
        let mut memory: Vec<i64> = Vec::new();

        let mut program: String = String::from("1,0,0,0,99");
        load_program(&program, &mut memory);
        let mut output: String = run_program(&mut memory);
        assert_eq!(output, String::from("2,0,0,0,99"));

        program = String::from("2,3,0,3,99");
        load_program(&program, &mut memory);
        output = run_program(&mut memory);
        assert_eq!(output, String::from("2,3,0,6,99"));

        program = String::from("2,4,4,5,99,0");
        load_program(&program, &mut memory);
        output = run_program(&mut memory);
        assert_eq!(output, String::from("2,4,4,5,99,9801"));

        program = String::from("1,1,1,4,99,5,6,0,99");
        load_program(&program, &mut memory);
        output = run_program(&mut memory);
        assert_eq!(output, String::from("30,1,1,4,2,5,6,0,99"));
    }
}