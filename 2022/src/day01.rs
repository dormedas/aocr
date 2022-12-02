fn get_sorted_elf_list(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = Vec::new();

    let mut calories_total = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories_total);
            calories_total = 0;
        } else {
            let calories: u32 = str::parse(line).unwrap();
            calories_total += calories;
        }
    }

    elves.sort();
    elves
}

pub fn part_one(input: &str) -> u32 {
    let elves = get_sorted_elf_list(input);
    *elves.last().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut elves = get_sorted_elf_list(input);
    let mut total = 0;
    for _ in 0..3 {
        total += elves.last().unwrap();
        elves.pop();
    }
    total
}
