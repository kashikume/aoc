struct Task {
    left : Vec<i32>,
    right : Vec<i32>,
}

impl Task {
    pub fn new() -> Task {
        Task { left: Vec::new(), right: Vec::new() }
    }

    pub fn read_input(&mut self, filename: &str) {
        let contents = std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        for line in contents.lines() {
            let mut tokens = line.split_whitespace();
            self.left.push(tokens.next().unwrap().parse::<i32>().unwrap());
            self.right.push(tokens.next().unwrap().parse::<i32>().unwrap());
        }
    }

    pub fn calculate(&mut self) {
        self.left.sort();
        self.right.sort();

        let size = self.left.len();

        let mut score = 0;
        let mut right_pos = 0;
        for i in 0..size {
            let mut on_right = 0;
            loop {
                if right_pos >= size || self.right[right_pos] > self.left[i] {
                    break;
                }
                else if self.right[right_pos] == self.left[i] {
                    on_right += 1;
                }
                right_pos += 1;
            }
            score += self.left[i] * on_right;
        }
        println!("{}", score);
    }
}


fn main() {
    let mut task = Task::new();
    task.read_input("input.txt");
    task.calculate();
}
