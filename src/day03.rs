use super::Problem;
use super::common;

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