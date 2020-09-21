extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
pub struct Circuit {
    x: i32,
    y: i32,
    wire: HashMap<i32, HashSet<i32>>,
}

impl Circuit {
    pub fn left(&mut self) {
        self.x -= 1;
        self.update();
    }

    pub fn right(&mut self) {
        self.x += 1;
        self.update();
    }

    pub fn up(&mut self) {
        self.y += 1;
        self.update();
    }

    pub fn down(&mut self) {
        self.y += 1;
        self.update();
    }

    pub fn at(&self, x: i32, y: i32) -> bool {
        let set = self.wire.get(&x);

        match set {
            None => false,
            Some(set) => match set.get(&y) {
                None => false,
                _ => true,
            },
        }
    }

    fn update(&mut self) {
        match self.wire.get(&self.x) {
            Some(_) => {}
            None => {
                let mut new_set = HashSet::<i32>::new();
                new_set.insert(self.y);
                self.wire.insert(self.x, new_set);
            }
        }
    }
}

pub fn new_circuit() -> Circuit {
    Circuit {
        x: 0,
        y: 0,
        wire: HashMap::new(),
    }
}

enum Move {
    Left(u32),
    Right(u32),
    Up(u32),
    Down(u32),
}

fn parse_move(foo: &str) -> Move {
    let re = Regex::new(r"^([LRUD])(\d+)$").unwrap();
    let caps = re.captures(foo).unwrap();
    let dir = caps.get(1).unwrap().as_str();
    let distance = u32::from_str(caps.get(2).unwrap().as_str()).unwrap();
    match dir {
        "L" => Move::Left(distance),
        "R" => Move::Right(distance),
        "U" => Move::Up(distance),
        "D" => Move::Down(distance),
        _ => panic!("What is {}", dir),
    }
}

fn split_moves(sequence: &str) -> Vec<&str> {
    sequence.split(",").collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_circuit() {
        let circuit = new_circuit();
        assert_eq!(circuit.x, 0);
    }

    #[test]
    fn test_right() {
        let mut circuit = new_circuit();
        circuit.right();
        assert_eq!(circuit.x, 1);
        assert_eq!(circuit.at(1, 0), true);
        assert_eq!(circuit.at(1, 1), false);
        assert_eq!(circuit.at(2, 0), false);

        circuit.right();
        assert_eq!(circuit.x, 2);
        assert_eq!(circuit.at(1, 0), true);
        assert_eq!(circuit.at(1, 1), false);
        assert_eq!(circuit.at(2, 0), true);

        circuit.left();
        assert_eq!(circuit.x, 1);
        assert_eq!(circuit.at(-1, 0), false);
        assert_eq!(circuit.at(1, 0), true);
        assert_eq!(circuit.at(1, 1), false);
        assert_eq!(circuit.at(2, 0), true);

        circuit.left();
        assert_eq!(circuit.x, 0);
        assert_eq!(circuit.at(-1, 0), false);
        assert_eq!(circuit.at(1, 0), true);
        assert_eq!(circuit.at(1, 1), false);
        assert_eq!(circuit.at(2, 0), true);

        circuit.left();
        assert_eq!(circuit.x, -1);
        assert_eq!(circuit.at(-1, 0), true);
        assert_eq!(circuit.at(1, 0), true);
        assert_eq!(circuit.at(1, 1), false);
        assert_eq!(circuit.at(2, 0), true);
    }

    #[test]
    fn test_at() {
        let circuit = new_circuit();
        assert_eq!(circuit.at(0, 0), false);
        assert_eq!(circuit.at(-1, 0), false);
        assert_eq!(circuit.at(1, -1), false);
    }

    #[test]
    fn test_parse_move() {
        assert!(matches!(parse_move("R75"), Move::Right(75)));
        assert!(matches!(parse_move("L1"), Move::Left(1)));
        assert!(matches!(parse_move("U11233"), Move::Up(11233)));
        assert!(matches!(parse_move("D0"), Move::Down(0)));
    }

    #[test]
    fn test_split_moves() {
        assert_eq!(split_moves("R75"), vec!["R75"]);
        assert_eq!(split_moves("R75,L32"), vec!["R75", "L32"]);
    }
}
