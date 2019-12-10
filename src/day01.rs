pub fn part_one() {
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

pub fn part_two() {
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
        fuel_requirement_total += generate_total_fuel_for_module(*i);
    }

    println!("{}", fuel_requirement_total);
}

pub fn generate_fuel_for_module(mass: i64) -> i64 {
    let mass_divided: i64 = mass / 3;
    mass_divided - 2
}

pub fn generate_total_fuel_for_module(mass: i64) -> i64 {
    let mass_divided: i64 = mass / 3;
    let result = mass_divided - 2;
    if result <= 0 {
        0
    } else {
        let fuel_mass: i64 = generate_total_fuel_for_module(result);
        result + fuel_mass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_fuel_calculation() {
        assert_eq!(generate_fuel_for_module(14), 2);
        assert_eq!(generate_fuel_for_module(1969), 654);
        assert_eq!(generate_fuel_for_module(100756), 33583);
    }

    #[test]
    fn verify_total_fuel_calculation() {
        assert_eq!(generate_fuel_for_module(14), 2);
        assert_eq!(generate_fuel_for_module(1969), 966);
        assert_eq!(generate_fuel_for_module(100756), 50346);
    }
}