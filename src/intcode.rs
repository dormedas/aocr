use std::io;

pub fn load_program(program: &String, memory: &mut Vec<i64>) {
    let program_strs: Vec<&str> = program.split(',').collect();

    memory.clear();

    for i in &program_strs {
        //println!("{}", i);
        let converted: i64 = i64::from_str_radix(i.trim(), 10).unwrap();
        memory.push(converted);
    }
}

fn dump_memory_as_program(memory: &mut Vec<i64>) -> String {
    let mut output: String = String::new();
    
    for j in memory {
        output.push_str(&(j.to_string() + ","));
    }

    // Remove the trailing comma
    output.remove(output.len() - 1);

    output
}

fn convert_opcode_to_mode_representation(opcode: i64) -> String {
    match opcode {
        1..=9 => {
            return format!("0000{}", opcode);
        },
        10..=99 => {
            return format!("000{}", opcode);
        },
        101..=999 => {
            return format!("00{}", opcode);
        },
        1001..=9999 => {
            return format!("0{}", opcode);
        }
        10001..=99999 => {
            return format!("{}", opcode);
        },
        _ => panic!()
    }
}

fn determine_value_from_mode(mode: i64, parameter_idx: usize, memory: &mut Vec<i64>) -> i64 {
    let pointer_index: usize = memory[parameter_idx] as usize;

    if mode == 1 { 
        memory[parameter_idx]
    } else { 
        memory[pointer_index] 
    }
}

// Runs program that is loaded into the passed in memory, 
pub fn run_program(memory: &mut Vec<i64>) -> String {
    let mut i: usize = 0;
    while i < memory.len() {
        let mut instruction_pointer_advance: usize = 1;

        let entire_opcode: i64 = memory[i];
        let entire_opcode_str: String = convert_opcode_to_mode_representation(entire_opcode);
        let opcode = i64::from_str_radix(entire_opcode_str.get(3..).unwrap(), 10).unwrap();
        let mut modes_array: [i64; 3] = [0; 3];
        for i in 0..3 {
            let mode = i64::from_str_radix(entire_opcode_str.get(i..i+1).unwrap(), 10).unwrap();
            modes_array[i] = mode;
        }
        modes_array.reverse();
        match opcode {
            1 => {
                let output_index: usize = memory[i+3] as usize;

                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                memory[output_index] = value_one + value_two;

                instruction_pointer_advance = 4;
            },
            2 => {
                let output_index: usize = memory[i+3] as usize;

                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                memory[output_index] = value_one * value_two;

                instruction_pointer_advance = 4;
            },
            3 => {
                let mut input = String::new();
                println!("Program asking for input: ");
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => (),
                    Err(error) => println!("error: {}", error),
                }
                let input_one_index: usize = memory[i+1] as usize;
                memory[input_one_index] = i64::from_str_radix(&input.trim(), 10).unwrap();
                instruction_pointer_advance = 2;
            },
            4 => {
                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);

                println!("{}", value_one);
                
                instruction_pointer_advance = 2;
            },
            5 => {
                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                if value_one != 0 {
                    instruction_pointer_advance = 0;
                    i = value_two as usize;
                } else {
                    instruction_pointer_advance = 3;
                }
            },
            6 => {
                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                if value_one == 0 {
                    instruction_pointer_advance = 0;
                    i = value_two as usize;
                } else {
                    instruction_pointer_advance = 3;
                }
            },
            7 => {
                let output_index: usize = memory[i+3] as usize;

                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                if value_one < value_two {
                    memory[output_index] = 1;
                } else {
                    memory[output_index] = 0;
                }

                instruction_pointer_advance = 4;
            },
            8 => {
                let output_index: usize = memory[i+3] as usize;

                let value_one: i64 = determine_value_from_mode(modes_array[0], i+1, memory);
                let value_two: i64 = determine_value_from_mode(modes_array[1], i+2, memory);

                if value_one == value_two {
                    memory[output_index] = 1;
                } else {
                    memory[output_index] = 0;
                }

                instruction_pointer_advance = 4;
            },
            99 => {
                break;
            },
            _ => panic!()
        }
        i += instruction_pointer_advance;
    }

    dump_memory_as_program(memory)
}

pub fn run(input_file: &str, noun: i64, verb: i64) -> i64 {
    let mut memory: Vec<i64> = Vec::new();

    let contents: String = super::common::read_file_to_string(input_file);

    load_program(&contents, &mut memory);
    if noun > -1 {
        memory[1] = noun;
    }
    if verb > -1 {
        memory[2] = verb;
    }

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