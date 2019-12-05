use days::one::*;

#[test]
fn verify_fuel_calculation() {
    assert_eq!(generate_fuel_for_module(12), 2);
    assert_eq!(generate_fuel_for_module(14), 2);
    assert_eq!(generate_fuel_for_module(1969), 654);
    assert_eq!(generate_fuel_for_module(100756), 33583);

}