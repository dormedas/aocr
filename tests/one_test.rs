use days::one::*;

#[test]
fn verify_fuel_calculation() {
    assert_eq!(generate_fuel_for_module(14), 2);
    assert_eq!(generate_fuel_for_module(1969), 966);
    assert_eq!(generate_fuel_for_module(100756), 50346);
}