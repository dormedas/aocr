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