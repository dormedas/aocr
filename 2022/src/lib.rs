pub mod day01;
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
