#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod common {
    use std::fs::File;
    // For read_to_string below
    use std::io::Read;

    pub fn read_file_to_string(file_name: &str) -> String {
        let mut f = File::open(file_name).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        contents
    }
}

pub mod one {
    pub fn run() {
        let contents: String = super::common::read_file_to_string("one_input.txt");

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
            fuel_requirement_total += generate_fuel_for_module(*i);
        }

        println!("{}", fuel_requirement_total);
    }

    pub fn generate_fuel_for_module(mass: i64) -> i64 {
        let mass_divided: i64 = mass / 3;
        let result = mass_divided - 2;
        if result <= 0 {
            0
        } else {
            let fuel_mass: i64 = generate_fuel_for_module(result);
            result + fuel_mass
        }
    }
}

pub mod two {
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

    pub fn run(noun: i64, verb: i64) -> i64 {
        let mut memory: Vec<i64> = Vec::new();

        let contents: String = super::common::read_file_to_string("two_input.txt");

        load_program(&contents, &mut memory);

        memory[1] = noun;
        memory[2] = verb;

        run_program(&mut memory);

        memory[0]
    }
}