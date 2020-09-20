use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Circuit {
    x: i32,
    y: i32,
    wire: HashMap<i32, HashSet<i32>>,
}

impl Circuit {
    pub fn right(&mut self) {
        self.x += 1;

        let set = self.wire.get(&self.x);
        match set {
            None => {
                let mut new_set = HashSet::<i32>::new();
                new_set.insert(self.y);
                self.wire.insert(self.x, new_set);
            },
            Some(_) => {}
        }
    }

    pub fn at(&self, x: i32, y: i32) -> bool {
        let set = self.wire.get(&x);

        match set {
            None => false,
            Some(set) => {
                match set.get(&y) {
                    None => false,
                    _ => true
                }
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
}
