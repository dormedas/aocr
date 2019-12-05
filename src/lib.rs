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
        mass_divided - 2
    }
}