use days::one::*;
use days::two::*;

#[test]
fn verify_fuel_calculation() {
    assert_eq!(generate_fuel_for_module(14), 2);
    assert_eq!(generate_fuel_for_module(1969), 966);
    assert_eq!(generate_fuel_for_module(100756), 50346);
}

#[test]
fn verify_intcode_computer() {
    let mut memory: Vec<i64> = Vec::new();

    let mut program: String = String::from("1,0,0,0,99");
    load_program(&program, &mut memory);
    let mut output: String = run_program(&mut memory);
    assert_eq!(output, String::from("2,0,0,0,99"));

    program = String::from("2,3,0,3,99");
    load_program(&program, &mut memory);
    output = run_program(&mut memory);
    assert_eq!(output, String::from("2,3,0,6,99"));

    program = String::from("2,4,4,5,99,0");
    load_program(&program, &mut memory);
    output = run_program(&mut memory);
    assert_eq!(output, String::from("2,4,4,5,99,9801"));

    program = String::from("1,1,1,4,99,5,6,0,99");
    load_program(&program, &mut memory);
    output = run_program(&mut memory);
    assert_eq!(output, String::from("30,1,1,4,2,5,6,0,99"));
}