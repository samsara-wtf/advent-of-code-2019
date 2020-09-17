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
    data.lines()
        .map(|line| i32::from_str(line).unwrap())
        .collect()
}

fn fuel_for_mass(kg: i32) -> i32 {
    kg / 3 - 2
}

fn solve() -> i32 {
    convert_to_numbers(read_lines())
        .iter()
        .map(|mass| fuel_for_mass(*mass))
        .sum()
}

fn solve2() -> i32 {
    convert_to_numbers(read_lines())
        .iter()
        .map(|mass| fuel_for_mass_and_fuel(*mass))
        .sum()
}

fn fuel_for_mass_and_fuel(kg: i32) -> i32 {
    let mut to_move = kg;
    let mut total_fuel = 0;

    loop {
        let fuel = fuel_for_mass(to_move);

        if fuel > 0 {
            total_fuel += fuel;
            to_move = fuel;
        }
        else {
            break total_fuel
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_for_fuel() {
        assert_eq!(2, fuel_for_mass_and_fuel(14));
        assert_eq!(2, fuel_for_mass_and_fuel(14));
        assert_eq!(966, fuel_for_mass_and_fuel(1969));
        assert_eq!(50346, fuel_for_mass_and_fuel(100756));
    }

    #[test]
    fn test_solution() {
        // assert_eq!(42, solve());
        assert_eq!(42, solve2());
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
