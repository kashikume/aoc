struct Task {
    list1 : Vec<i32>,
    list2 : Vec<i32>,
}

impl Task {
    pub fn new() -> Task {
        Task { list1: Vec::new(), list2: Vec::new() }
    }

    pub fn read_input(&mut self, filename: &str) {
        let contents = std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        for line in contents.lines() {
            let mut tokens = line.split_whitespace();
            self.list1.push(tokens.next().unwrap().parse::<i32>().unwrap());
            self.list2.push(tokens.next().unwrap().parse::<i32>().unwrap());
        }
    }

    pub fn calculate(&mut self) {
        self.list1.sort();
        self.list2.sort();

        let size = self.list1.len();

        let mut dist = 0;
        for i in 0..size {
            dist += (self.list1[i] - self.list2[i]).abs();
        }
        println!("{}", dist);
    }
}


fn main() {
    let mut task = Task::new();
    task.read_input("input.txt");
    task.calculate();
}
