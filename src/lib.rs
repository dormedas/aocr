#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod one {
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