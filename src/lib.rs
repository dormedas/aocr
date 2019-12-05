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
    #[derive(Debug)]
    pub struct Intcode {
        pub program_location: i64,
        pub opcode: i64,
        pub input_one: i64,
        pub input_two: i64,
        pub output: i64
    }

    pub fn run() {
        let contents: String = super::common::read_file_to_string("two_input.txt");

        let contents_str: Vec<&str> = contents.split(',').collect();

        let mut memory: Vec<i64> = Vec::new();

        for i in &contents_str {
            println!("{}", i);
            let converted: i64 = i64::from_str_radix(i.trim(), 10).unwrap();
            memory.push(converted);
        }

        memory[1] = 12;
        memory[2] = 2;

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

        println!("---");

        let mut output: String = String::new();
        
        for i in memory {
            output.push_str(&(i.to_string() + ","));
        }

        println!("{}", output);

        let intcode: Intcode = Intcode {
            program_location: 0,
            opcode: 1,
            input_one: 20,
            input_two: 30,
            output: 0,
        };
    }
}