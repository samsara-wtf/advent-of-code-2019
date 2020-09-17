use std::fs;
use std::str::FromStr;

pub fn hola() {
    println!("hi from day01");

    let contents = fs::read_to_string("src/day01.input").expect("alas, poor Yorick");

    println!("Got this: {}", contents);
}

fn read_lines() -> std::string::String {
    fs::read_to_string("src/day01.input").expect("barf")
}

fn convert_to_numbers(data: std::string::String) -> Vec<i32> {
    data.lines().map(|line| i32::from_str(line).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_numbers() {
        let numbers = convert_to_numbers(read_lines());
        assert_eq!(133260, numbers[0]);
        assert_eq!(97852, numbers[numbers.len() - 1])
    }

    // #[test]
    // fn test_read_lines() {
    //     let a = read_lines();
    //     let b = read_lines();
    //     assert_eq!(a, b);
    // }
}
