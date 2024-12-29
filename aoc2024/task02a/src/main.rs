struct Task {

}

impl Task {
    pub fn new() -> Task {
        Task {  }
    }

    pub fn check(line: &Vec<i32>, ignore: i32) -> bool {
        let mut inc = 0;
        let mut last = -1;
        let mut is_safe = true;

        for index in 0..line.len() {
            let val = line[index];
            if index as i32 == ignore {
                continue;
            }
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
        is_safe
    }

    pub fn check_input(&mut self, filename: &str) {
        let contents = std::fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut output = 0;

        for line in contents.lines() {
            let tokens = line.split_whitespace();

            let mut data = Vec::new();

            for token in tokens {
                let val = token.parse::<i32>().unwrap();
                data.push(val);

            }
            let mut is_safe: bool = false;

            for index in -1..data.len() as i32 {
                if Task::check(&data, index) {
                    is_safe = true;
                    break;
                }
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
