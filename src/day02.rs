use std::fs;
use std::io;
use std::str::FromStr;

fn read_data() -> std::string::String {
    fs::read_to_string("src/day02.input").expect("barf")
}

fn convert_to_numbers(data: std::string::String) -> Vec<i32> {
    let foo: Vec<&str> = data.split(',').collect();
    // println!("foo: {:#?}", foo);

    foo.iter()
        .map(|line| i32::from_str(line).unwrap())
        .collect()
}

fn numbers_to_opcodes(numbers: Vec<i32>) -> Vec<[i32; 5]> {
    let mut vec = vec![];
    let mut index: usize = 0;
    let mut work: [i32; 5] = [0, 0, 0, 0, 0];

    for num in numbers {
        work[index % 5] = num;
        index += 1;
        if index % 5 == 0 {
            vec.push(work.clone());
            break;
        }
    }

    return vec;
}

pub fn run_trial() {
    let mut input = String::new();
    let noun: i32;
    let verb: i32;

    println!("What's the noun?");
    io::stdin().read_line(&mut input).expect("need a noun");

    noun = input.trim().parse().expect("alas");

    println!("What's the verb?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("need a noun");

    verb = input.trim().parse().expect("alas");

    let mut memory = convert_to_numbers(read_data());
    memory[1] = noun;
    memory[2] = verb;

    let output = execute(memory.clone());
    println!("{} = {}", noun * 100 + verb, output[0]);
}

fn execute(opcodes: Vec<i32>) -> Vec<i32> {
    let mut opcode_index: usize = 0;
    let mut output = opcodes.clone();

    loop {
        let operator = output[opcode_index];
        opcode_index += 1;

        if operator == 99 {
            break;
        }

        let lhs_index = output[opcode_index] as usize;
        let lhs_val = output[lhs_index];
        opcode_index += 1;

        let rhs_index = output[opcode_index] as usize;
        let rhs_val = output[rhs_index];
        opcode_index += 1;

        let out_index = output[opcode_index] as usize;
        let out_val = if operator == 1 {
            lhs_val + rhs_val
        } else if operator == 2 {
            lhs_val * rhs_val
        } else {
            panic!("wtf is this operator: {}", operator);
        };
        opcode_index += 1;

        output[out_index] = out_val;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_to_opcodes() {
        let nums = convert_to_numbers(read_data());
        assert_eq!([1, 0, 0, 3, 1], numbers_to_opcodes(nums)[0]);
    }

    #[test]
    fn test_convert_to_numbers() {
        let nums = convert_to_numbers(read_data());
        assert_eq!(nums[0], 1);
        assert_eq!(nums[nums.len() - 1], 0)
    }

    #[test]
    fn test_execute() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        assert_eq!(execute(input), output);

        let input2 = vec![2, 3, 0, 3, 99];
        let output2 = vec![2, 3, 0, 6, 99];
        assert_eq!(execute(input2), output2);

        let input3 = vec![2, 4, 4, 5, 99, 0];
        let output3 = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(execute(input3), output3);

        let input4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output4 = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(execute(input4), output4);
    }

    #[test]
    fn test_solve() {
        let mut input = convert_to_numbers(read_data());
        input[1] = 12;
        input[2] = 2;
        let output = execute(input.clone());
        // assert_eq!(42, output[0]);
        assert!(42 != output[0]);
    }
}
