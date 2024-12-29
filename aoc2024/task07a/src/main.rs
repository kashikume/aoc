use std::fs;
use std::time;

struct Equation {
    result: i64,
    data: Vec<i64>,
}

impl Equation {
    fn from_string(input: &str) -> Equation {
        let mut parts=input.split(':');
        let result = parts.next().unwrap().parse::<i64>().unwrap();
        let data = parts.next().unwrap().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        Equation { result, data }
    }

    fn check(&self, operators:&Vec<char>) -> bool {
        let mut result = self.data[0];
        for(pos, operator) in operators.iter().enumerate() {
            match operator {
                '+' => result += self.data[pos+1],
                '*' => result *= self.data[pos+1],
                _ => panic!("Unknown operator"),
            }
            if result > self.result {
                return false;
            }
        }
        return self.result == result;
    }

    fn solve(&self) -> bool {
        let operators = Operators::new((self.data.len()-1) as i32);
        for operator in operators {
            if self.check(&operator) {
                return true;
            }
        }
        return false;
    }
}

struct Operators {
    value: i32,
    max_value: i32,
    size: i32,
}

impl Operators {
    fn new(size:i32) -> Operators {
        Operators {
            value: 0,
            max_value: 2i32.pow(size as u32),
            size: size,
        }
    }
}

impl Iterator for Operators {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Vec<char>> {
        if self.value == self.max_value {
            return None;
        }
        
        let mut result = Vec::new();
        let mut value = self.value;
        for _ in 0..self.size {
            match value % 2 {
                0 => result.push('+'),
                1 => result.push('*'),
                _ => panic!("Unknown operator"),
            }
            value /= 2;
        }
        self.value += 1;
        Some(result)
    }
}

struct Task {
    equations: Vec<Equation>,
}

impl Task {
    fn from_file(filen_name: &str) -> Task {
        let content = fs::read_to_string(&filen_name).expect("Error reading file.");
        let mut equations = Vec::new();
        let lines = content.lines();
        for line in lines {
            let equation = Equation::from_string(line);
            equations.push(equation);
        }
        Task { equations }
    }

    fn solve(&self) -> i64 {
        let mut result = 0;
        for equation in &self.equations {
            if equation.solve() {
                result += equation.result;
            }
        }
        result
    }
}

fn main() {
    let start = time::Instant::now();

    let task = Task::from_file("input.txt");
    println!("Result: {}", task.solve());
    println!("Time elapsed: {:?}", start.elapsed());
}
