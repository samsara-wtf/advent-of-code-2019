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

fn fuel_for_mass(kg: i32) -> i32 {
    kg / 3 - 2
}

fn solve() -> i32 {
    convert_to_numbers(read_lines())
        .iter()
        .map( |mass| fuel_for_mass(*mass) )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(42, solve());
    }

    #[test]
    fn test_convert_to_numbers() {
        let numbers = convert_to_numbers(read_lines());
        assert_eq!(133260, numbers[0]);
        assert_eq!(97852, numbers[numbers.len() - 1])
    }

    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(2, fuel_for_mass(12));
        assert_eq!(2, fuel_for_mass(14));
        assert_eq!(654, fuel_for_mass(1969));
        assert_eq!(33583, fuel_for_mass(100756));
    }
}
