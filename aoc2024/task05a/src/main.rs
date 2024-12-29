use std::fs;

#[derive(Debug)]
struct Rule {
    first: i32,
    last: i32,
}

#[derive(Debug)]
struct Instruction {
    pages: Vec<i32>,
}

#[derive(Debug)]
struct Solution {
    rules: Vec<Rule>,
    instructions: Vec<Instruction>,
}

impl Instruction {
    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        for rule in rules {
            if self.pages.contains(&rule.first) && self.pages.contains(&rule.last) {
                if self.pages.iter().position(|x| x == &rule.first) > self.pages.iter().position(|x| x == &rule.last) {
                    return false;
                }
            }
        }
        true
    }

    fn middle(&self) -> i32 {
        self.pages[self.pages.len()/2 as usize]
    }
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            rules: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn from_file(file_name: &str) -> Solution {
        let content = fs::read_to_string(file_name).expect("Error reading file");
        let mut solution = Solution::new();

        for line in content.lines() {
            if line.is_empty() {
                continue;
            }
            if line.contains("|") {
                let mut parts = line.split("|");
                let mut rule = Rule {
                    first: parts.next().unwrap().parse().unwrap(),
                    last: parts.next().unwrap().parse().unwrap(),
                };
                solution.rules.push(rule);
            } else {
                let mut instruction = Instruction {
                    pages: line.split(",").map(|x| x.parse().unwrap()).collect(),
                };
                solution.instructions.push(instruction);
            }
        }

        solution
    }

    pub fn calculate(&self) -> i32 {
        let mut result = 0;
        for instruction in &self.instructions {
            if instruction.is_valid(&self.rules) {
                result += instruction.middle();
            }
        }
        result
    }
}

fn main() {
    let solution = Solution::from_file("input.txt");
    println!("{:?}", solution);
    println!("Result: {}", solution.calculate());
}
