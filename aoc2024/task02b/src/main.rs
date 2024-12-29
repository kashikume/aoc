struct Task {

}

impl Task {
    pub fn new() -> Task {
        Task {  }
    }

    pub fn check_input(&mut self, filename: &str) {
        let contents = std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut output = 0;

        for line in contents.lines() {
            let tokens = line.split_whitespace();

            let mut inc = 0;
            let mut last = -1;
            let mut is_safe = true;

            for token in tokens {
                let val = token.parse::<i32>().unwrap();
                if last != -1 {
                    let diff = val - last;
                    if inc == 0 && diff != 0 {
                        inc = diff / diff.abs();
                    }
                    if diff == 0 || inc != diff / diff.abs() || diff.abs() > 3 {
                        is_safe = false;
                        break;
                    }
                }
                last = val;
            }
            if is_safe {
                output += 1;
            }
        }
        println!("Output: {}", output);
    }
}


fn main() {
    let mut task = Task::new();
    task.check_input("input.txt");
}
