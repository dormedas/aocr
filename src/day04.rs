pub fn part_one() -> String {
    let minimum_range: i64 = 248345;
    let maximum_range: i64 = 746315;

    let mut count: i64 = 0;
    for i in minimum_range..maximum_range {
        let num_as_str: String = i.to_string();
        if is_ascending(&num_as_str) && has_doubles_at_all(&num_as_str) {
            count += 1;
        }
    }

    count.to_string()
}

pub fn part_two() -> String {
    let minimum_range: i64 = 248345;
    let maximum_range: i64 = 746315;

    let mut count: i64 = 0;
    for i in minimum_range..maximum_range {
        let num_as_str: String = i.to_string();
        if is_ascending(&num_as_str) && has_doubles_at_all(&num_as_str) && has_doubles_exactly(&num_as_str) {
            count += 1;
        }
    }

    count.to_string()
}

pub fn is_in_range(input: i64, minimum: i64, maximum: i64) -> bool {
    input >= minimum && input <= maximum
}

pub fn is_ascending(input: &str) -> bool {
    let mut max_num: i64 = 0;

    for i in 0..input.len() {
        let slice: &str = input.get(i..i+1).unwrap();
        //println!("{}", slice);
        let num: i64 = i64::from_str_radix(slice, 10).unwrap();
        if num < max_num {
            return false;
        } else {
            max_num = num;
        }
    }

    true
}

pub fn has_doubles_at_all(input: &str) -> bool {
    for i in 0..input.len() - 1 {
        let slice: &str = input.get(i..i+2).unwrap();
        //println!("{}", slice);
        let num: i64 = i64::from_str_radix(slice, 10).unwrap();
        match num {
            00 | 11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 | 99 => return true,
            _ => (),
        }
    }

    false
}

pub fn has_doubles_exactly(input: &str) -> bool {
    let mut i: usize = 0;

    while i < input.len() {
        let char_to_count: &str = input.get(i..i+1).unwrap();
        let mut j: usize = i + 1;
        while j < input.len() {
            if input.get(j..j+1).unwrap() == char_to_count {
                j += 1;
            } else {
                break;
            }
        }

        if j - i == 2 {
            return true;
        }

        i += j - i;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubles() {
        assert_eq!(has_doubles_at_all("123356"), true);
        assert_eq!(has_doubles_at_all("123789"), false);
    }

    #[test]
    fn test_ascending() {
        assert_eq!(is_ascending("123456"), true);
    }

    #[test]
    fn test_part2() {
        let mut input = 112233;
        let mut num_as_str: String = input.to_string();
        assert_eq!(is_ascending(&num_as_str) && has_doubles_at_all(&num_as_str) && has_doubles_exactly(&num_as_str), true);

        input = 123444;
        num_as_str = input.to_string();
        assert_eq!(is_ascending(&num_as_str) && has_doubles_at_all(&num_as_str) && has_doubles_exactly(&num_as_str), false);

        input = 111122;
        num_as_str = input.to_string();
        assert_eq!(is_ascending(&num_as_str) && has_doubles_at_all(&num_as_str) && has_doubles_exactly(&num_as_str), true);
    }
}