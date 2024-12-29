use std::fs;

#[derive(Debug)]
struct Program {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<i64>,
    pc: usize,
}

impl Program {
    fn new(file_name: &str) -> Program {
        let contents = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file");
        
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = Vec::new();
        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }
            let mut parts = line.split_whitespace();
            match parts.next().unwrap() {
                "Register" => {
                    match parts.next().unwrap() {
                        "A:" => a = parts.next().unwrap().parse().unwrap(),
                        "B:" => b = parts.next().unwrap().parse().unwrap(),
                        "C:" => c = parts.next().unwrap().parse().unwrap(),
                        _ => panic!("Invalid register"),
                    }
                }
                "Program:" => {
                    program = parts.next().unwrap().split(',').map(|x| x.parse().unwrap()).collect();
                }
                _ => panic!("Invalid input"),
            }
        }
        Program {
            a,
            b,
            c,
            program,
            pc: 0,
        }
    }

    fn read_instruction(&mut self) -> i64 {
        let instr = self.program[self.pc];
        self.pc+=1;
        instr
    }

    fn read_combo_operand(&mut self) -> i64 {
        let operand = self.program[self.pc];
        self.pc+=1;
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand"),
        }
    }

    fn read_literal_operand(&mut self) -> i64 {
        let operand = self.program[self.pc];
        self.pc+=1;
        operand
    }

    fn execute(&mut self) {
        let mut coma = false;
        while self.pc < self.program.len() {
            let instr = self.read_instruction();

            match instr {
                0 => { // adv
                    let operand = self.read_combo_operand();
                    self.a = self.a / (2i64.pow(operand as u32));
                    }
                1 => { // bxl
                    let operand = self.read_literal_operand();
                    self.b = self.b ^ operand;
                    }
                2 => { // bst
                    let operand = self.read_combo_operand();
                    self.b = operand % 8;
                }
                3 => { // jnz
                    let operand = self.read_literal_operand();
                    if self.a != 0 {
                        self.pc = operand as usize;
                    }
                }
                4 => { // bxc
                    let _operand = self.read_literal_operand();
                    self.b = self.b ^ self.c;
                }
                5 => { // out
                    let operand = self.read_combo_operand();
                    if coma {
                        print!(",");
                    }
                    print!("{}", operand % 8);
                    coma = true;
                }
                6 => { // bdv
                    let operand = self.read_combo_operand();
                    self.b = self.a / (2i64.pow(operand as u32));
                }
                7 => { // cdv
                    let operand = self.read_combo_operand();
                    self.c = self.a / (2i64.pow(operand as u32));
                }
                _ => { panic!("Invalid instruction"); }
            }
        }
        println!("");
    }
}

fn main() {
    let mut program = Program::new("input.txt");
    println!("{:?}", program);
    program.execute();
}
