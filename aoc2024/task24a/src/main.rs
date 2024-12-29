use std::fs;
use std::collections::HashMap;

enum Operator {
    op_and,
    op_or,
    op_xor,
}

struct Operation {
    operator: Operator,
    input1: String,
    input2: String,
    output: String,
}

fn main() {
    let mut operations = HashMap::new();
    let mut values: HashMap<String, i32> = HashMap::new();
    let mut stack = Vec::new();
    let mut output = Vec::new();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut lines = contents.lines();

    let mut reading_values = true;
    for line in lines {
        if line.is_empty() {
            reading_values = false;
        }
        else if reading_values {
            let parts: Vec<&str> = line.split(": ").collect();
            values.insert(parts[0].to_string(), parts[1].parse().unwrap());
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[4].starts_with("z") {
                output.push(parts[4].to_string());
            }
            let op = match parts[1] {
                "AND" => Operator::op_and,
                "OR" => Operator::op_or,
                "XOR" => Operator::op_xor,
                _ => {
                    panic!("Unknown operation: {}", parts[1]);
                }
            };
            operations.insert(parts[4].to_string(), Operation{operator: op, input1: parts[0].to_string(), input2: parts[2].to_string(), output: parts[4].to_string()});
        }
    }

    output.sort();
    stack = output.clone();
    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        if values.contains_key(&current) {
            continue;
        }

        let operation = operations.get(&current).unwrap();
        if !values.contains_key(&operation.input1) || !values.contains_key(&operation.input2) {
            stack.push(current.clone());
            if !values.contains_key(&operation.input1) {
                stack.push(operation.input1.clone());
            }
            if !values.contains_key(&operation.input2) {
                stack.push(operation.input2.clone());
            }
        }
        else {
            let input1 = values.get(&operation.input1).unwrap();
            let input2 = values.get(&operation.input2).unwrap();
            let result = match operation.operator {
                Operator::op_and => input1 & input2,
                Operator::op_or => input1 | input2,
                Operator::op_xor => input1 ^ input2,
            };
            values.insert(operation.output.clone(), result);
        }
    }

    let mut part1_output = String::new();
    for key in output {
        println!("{}", values.get(&key).unwrap());
        part1_output = format!("{}{}", values.get(&key).unwrap(),part1_output);
    }
    let part1_number = u64::from_str_radix(&part1_output, 2).unwrap();
    println!("{}", part1_number);
}
