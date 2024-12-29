use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Operator {
    OpAnd,
    OpOr,
    OpXor,
}

#[derive(Clone)]
struct Operation {
    operator: Operator,
    input1: String,
    input2: String,
    output: String,
}

struct Solution {
    values: HashMap<String, i32>,
    operations: HashMap<String, Operation>,
    output_z: Vec<String>,
    input_x: Vec<String>,
    input_y: Vec<String>,
    swaps: Vec<String>,
}

impl Solution {
    fn new(file_name: &str) -> Self {
        let mut operations = HashMap::new();
        let mut values: HashMap<String, i32> = HashMap::new();
        let mut output_z = Vec::new();
        let mut input_x = Vec::new();
        let mut input_y = Vec::new();
    
        let contents = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file");
        let lines = contents.lines();
    
        let mut reading_values = true;
        for line in lines {
            if line.is_empty() {
                reading_values = false;
            }
            else if reading_values {
                let parts: Vec<&str> = line.split(": ").collect();
                values.insert(parts[0].to_string(), parts[1].parse().unwrap());
                if parts[0].starts_with("x") {
                    input_x.push(parts[0].to_string());
                }
                if parts[0].starts_with("y") {
                    input_y.push(parts[0].to_string());
                }
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                if parts[4].starts_with("z") {
                    output_z.push(parts[4].to_string());
                }
                let op = match parts[1] {
                    "AND" => Operator::OpAnd,
                    "OR" => Operator::OpOr,
                    "XOR" => Operator::OpXor,
                    _ => {
                        panic!("Unknown operation: {}", parts[1]);
                    }
                };
                operations.insert(parts[4].to_string(), Operation{operator: op, input1: parts[0].to_string(), input2: parts[2].to_string(), output: parts[4].to_string()});
            }
        }
    
        output_z.sort();
        input_x.sort();
        input_y.sort();
        let swaps = Vec::new();

        Solution{values, operations, output_z, input_x, input_y, swaps}
    }

    fn calculate(&mut self) {
        let mut stack = self.output_z.clone();
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            if self.values.contains_key(&current) {
                continue;
            }
    
            let operation = self.operations.get(&current).unwrap();
            if !self.values.contains_key(&operation.input1) || !self.values.contains_key(&operation.input2) {
                stack.push(current.clone());
                if !self.values.contains_key(&operation.input1) {
                    stack.push(operation.input1.clone());
                }
                if !self.values.contains_key(&operation.input2) {
                    stack.push(operation.input2.clone());
                }
            }
            else {
                let input1 = self.values.get(&operation.input1).unwrap();
                let input2 = self.values.get(&operation.input2).unwrap();
                let result = match operation.operator {
                    Operator::OpAnd => input1 & input2,
                    Operator::OpOr => input1 | input2,
                    Operator::OpXor => input1 ^ input2,
                };
                self.values.insert(operation.output.clone(), result);
            }
        }
    }

    fn print_values(&self) {
        let x = print_value("x=  ",&self.input_x, &self.values);
        let y = print_value("y=  ",&self.input_y, &self.values);
                     print_value("z= ",&self.output_z, &self.values);
    
        let sum = format!("{:#b}", x+y).replace("0b", "");

        println!("   {} = {}", sum, x+y);
    }

    fn find(&mut self, what: &str, input1: &String, input2: &String, op: &Operator) -> String {
        for (result, operation) in &self.operations {
            if operation.operator == *op {
                if (operation.input1 == *input1 && operation.input2 == *input2) || (operation.input1 == *input2 && operation.input2 == *input1) {
                    return result.clone();
                }
            }
        }
        println!("Error finding {} : {} {:?} {} not found\nAvailable:", what, input1, op, input2);
        let mut to_swap = Vec::new();
        for (result, operation) in &self.operations {
            if operation.operator == *op {
                if operation.input1 == *input1 || operation.input2 == *input2 || operation.input1 == *input2 || operation.input2 == *input1 {
                    println!("{} {:?} {} => {}", operation.input1, operation.operator, operation.input2, result);
                    let mut all = vec![operation.input1.clone(), operation.input2.clone(), (*input1).to_string(), (*input2).to_string(), "zzzz".to_string()];
                    all.sort();
                    
                    for i in 0..all.len()-1 {
                        if all[i] != all[i+1] {
                            to_swap.push(all[i].clone());
                        }
                    }
                    break;
                }
            }
        }

        self.swap(&(to_swap[0]), &(to_swap[1]));
        String::new()
    }

    fn verify(&mut self) -> bool {
        let mut xor1;
        let mut to_next = self.find("to_next", &"x00".to_string(), &"y00".to_string(), &Operator::OpAnd);
        let mut to_next1;
        let mut to_next2;
        let mut xor2;


         for i in 1..45 {
            let x = format!("x{:02}", i);
            let y = format!("y{:02}", i);
            let z = format!("z{:02}", i);

            xor1 =  self.find("xor1", &x, &y, &Operator::OpXor);
            to_next1 = self.find("to_next1",&x, &y, &Operator::OpAnd);
            to_next2 = self.find("to_next2", &xor1, &to_next, &Operator::OpAnd);
            xor2 =  self.find("xor2", &to_next, &xor1, &Operator::OpXor);
            to_next = self.find("to_next", &to_next1, &to_next2, &Operator::OpOr);
            if z != xor2 {
                println!("Error xor2 : {} {} {} {}", x, y, z, xor2);
            }

            println!("{}: xor1={}, to_next1={}, to_next2={}, xor2={}, to_next={}", i, xor1, to_next1, to_next2, xor2, to_next);
            println!("----");
            if xor1.is_empty() || to_next1.is_empty() || to_next2.is_empty() || xor2.is_empty() || to_next.is_empty() {
                return false;
            }
        }
        true
    }

    fn swap(&mut self, x1: &String, x2: &String) {
        let mut x1_val = (*self.operations.get(x1).unwrap()).clone();
        let mut x2_val = (*self.operations.get(x2).unwrap()).clone();

        x1_val.output = x2.clone();
        x2_val.output = x1.clone();

        self.operations.insert(x1.clone(), x2_val);
        self.operations.insert(x2.clone(), x1_val);

        self.swaps.push(x1.clone());
        self.swaps.push(x2.clone());
    }
}

fn print_value(begin: &str, keys: &Vec<String>, values: &HashMap<String, i32>) -> u64 {
    print!("{}", begin);
    let mut part1_output = String::new();
    for key in keys {
        part1_output = format!("{}{}", values.get(key).unwrap(),part1_output);
    }
    let part1_number = u64::from_str_radix(&part1_output, 2).unwrap();
    println!("{} = {}", part1_output, part1_number);
    part1_number
}

fn main() {
    let mut solution = Solution::new("input.txt");
    
    //let mut swaps = ["z10", "mwk", "z18", "qgd" , "jmh", "hsw", "z33", "gqp" ];

   // for i in (0..swaps.len()).step_by(2) {
   //     solution.swap(&swaps[i].to_string(), &swaps[i+1].to_string());
   // }

    while !solution.verify() {}
    solution.calculate();
    solution.print_values();

    solution.swaps.sort();
    for s in solution.swaps {
        print!("{},",s);
    }
}
