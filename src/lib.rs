#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait Problem {
    fn part_one(&mut self, input: &str) -> String;
    fn part_two(&mut self, input: &str) -> String;
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

pub struct Day03 {
    pub wire_head: (i64, i64, i64),
}

impl Day03 {
    fn process_movement(&mut self, movement: &str) -> Vec<(i64, i64, i64)> {
        let (direction, num) = movement.split_at(1);

        println!("direction: {}, num: {}", direction, num);
        let num_i64: i64 = i64::from_str_radix(num, 10).unwrap();
        let num_loop_max: i64 = num_i64 + 1;
        let mut vec_to_return: Vec<(i64, i64, i64)> = Vec::new();
        match direction {
            "U" => {
                for i in 0..num_loop_max {
                    let pos = (self.wire_head.0, self.wire_head.1 + i, self.wire_head.2 + i);
                    vec_to_return.push(pos);
                    if i == num_i64 {
                        self.wire_head = (self.wire_head.0, self.wire_head.1 + i, self.wire_head.2 + i);
                    }
                }
            },
            "D" => {
                for i in 0..num_loop_max {
                    let pos = (self.wire_head.0, self.wire_head.1 - i, self.wire_head.2 + i);
                    vec_to_return.push(pos);
                    if i == num_i64 {
                        self.wire_head = (self.wire_head.0, self.wire_head.1 - i, self.wire_head.2 + i);
                    }
                }            
            },
            "L" => {
                for i in 0..num_loop_max {
                    let pos = (self.wire_head.0 - i, self.wire_head.1, self.wire_head.2 + i);
                    vec_to_return.push(pos);
                    if i == num_i64 {
                        self.wire_head = (self.wire_head.0 - i, self.wire_head.1, self.wire_head.2 + i);
                    }
                } 
            },
            "R" => {
                for i in 0..num_loop_max {
                    let pos = (self.wire_head.0 + i, self.wire_head.1, self.wire_head.2 + i);
                    vec_to_return.push(pos);
                    if i == num_i64 {
                        self.wire_head = (self.wire_head.0 + i, self.wire_head.1, self.wire_head.2 + i);
                    }
                } 
            },
            _ => panic!()
        }
        vec_to_return
    }

    fn calculate_manhattan_distance(location: (i64, i64, i64)) -> i64 {
        location.0.abs() + location.1.abs()
    }

    fn calculate_latency(latency_one: i64, latency_two: i64) -> i64 {
        latency_one + latency_two
    }
}

impl Problem for Day03 {
    fn part_one(&mut self, input: &str) -> String {
        let mut contents: String = common::read_file_to_string("three_input.txt");
        let mut indices_to_remove: Vec<usize> = Vec::new();

        let mut idx = 0;
        for c in contents.chars() {
            if c == '\n' {
                indices_to_remove.push(idx);
            }
            idx +=  1;
        }

        for i in indices_to_remove.iter().rev() {
            contents.remove(*i);
        }

        let (wire_one, wire_two) = contents.split_at(indices_to_remove[0]);
        let wire_one_movement_strs: Vec<&str> = wire_one.split(',').collect();
        let wire_two_movement_strs: Vec<&str> = wire_two.split(',').collect();

        let mut wire_one_positions: Vec<(i64, i64, i64)> = Vec::new();
        let mut wire_two_positions: Vec<(i64, i64, i64)> = Vec::new();
        for i in wire_one_movement_strs {
            let mut wire_pos = self.process_movement(i);
            wire_one_positions.append(&mut wire_pos);
        }
        self.wire_head = (0, 0, 0);
        for i in wire_two_movement_strs {
            let mut wire_pos = self.process_movement(i);
            wire_two_positions.append(&mut wire_pos);
        }

        let mut matches: Vec<(i64, i64, i64)> = Vec::new();

        for i in &wire_one_positions {
            for j in &wire_two_positions {
                if i.0 == j.0 && i.1 == j.1 {
                    if i.0 == 0 && i.1 == 0 {
                        continue;
                    }
                    matches.push(*i);
                }
            }
        }

        let mut minimum_distance: i64 = i64::max_value();

        for i in matches {
            let manhattan_distance: i64 = Day03::calculate_manhattan_distance(i);
            if manhattan_distance < minimum_distance {
                minimum_distance = manhattan_distance;
            }
        }

        println!("{}", minimum_distance);

        minimum_distance.to_string()
    }

    fn part_two(&mut self, input: &str) -> String {
        let mut contents: String = common::read_file_to_string("three_input.txt");
        let mut indices_to_remove: Vec<usize> = Vec::new();

        let mut idx = 0;
        for c in contents.chars() {
            if c == '\n' {
                indices_to_remove.push(idx);
            }
            idx +=  1;
        }

        for i in indices_to_remove.iter().rev() {
            contents.remove(*i);
        }

        let (wire_one, wire_two) = contents.split_at(indices_to_remove[0]);
        let wire_one_movement_strs: Vec<&str> = wire_one.split(',').collect();
        let wire_two_movement_strs: Vec<&str> = wire_two.split(',').collect();

        let mut wire_one_positions: Vec<(i64, i64, i64)> = Vec::new();
        let mut wire_two_positions: Vec<(i64, i64, i64)> = Vec::new();
        for i in wire_one_movement_strs {
            let mut wire_pos = self.process_movement(i);
            wire_one_positions.append(&mut wire_pos);
        }
        self.wire_head = (0, 0, 0);
        for i in wire_two_movement_strs {
            let mut wire_pos = self.process_movement(i);
            wire_two_positions.append(&mut wire_pos);
        }

        let mut latencies: Vec<i64> = Vec::new();

        for i in &wire_one_positions {
            for j in &wire_two_positions {
                if i.0 == j.0 && i.1 == j.1 {
                    if i.0 == 0 && i.1 == 0 {
                        continue;
                    }
                    latencies.push(Day03::calculate_latency(i.2, j.2));
                }
            }
        }

        let mut minimum_latency: i64 = i64::max_value();

        for i in latencies {
            if i < minimum_latency {
                minimum_latency = i;
            }
        }

        println!("{}", minimum_latency);

        minimum_latency.to_string()    
    }
}