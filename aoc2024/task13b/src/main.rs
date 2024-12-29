use std::time;
use std::fs;

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

struct Solution {
    machines : Vec<Machine>,
}

impl Machine {

    // a*ax + b*bx = px
    // b*ay + b*by = py
    fn solve(&self) -> i64 {
        let w = self.ax * self.by - self.ay * self.bx;
        let wa = self.px * self.by - self.py * self.bx;
        let wb = self.ax * self.py - self.ay * self.px;

        let a = wa / w;
        let b = wb / w;

        if a < 0 || b < 0 { return 0; }
        if a * self.ax + b * self.bx != self.px { return 0; }
        if a * self.ay + b * self.by != self.py { return 0; }
        return a * 3 + b;

    }
}

impl Solution {
    fn from_file(file: &str) -> Solution {
        let contents = fs::read_to_string(file)
            .expect("Something went wrong reading the file");

        let mut machines = Vec::new();
        let mut lines = contents.lines();
        loop {
            let n = lines.next();
            if n == None { break; }
            if n.unwrap().is_empty(){ continue; }

            let l = n.unwrap();
            let l = l.replace("Button A: X+", "");
            let l = l.replace(", Y+", " ");
            let mut tokens = l.split_whitespace();
            let ax = tokens.next().unwrap().parse::<i64>().unwrap();
            let ay = tokens.next().unwrap().parse::<i64>().unwrap();

            let n = lines.next();
            let l = n.unwrap();
            let l = l.replace("Button B: X+", "");
            let l = l.replace(", Y+", " ");
            let mut tokens = l.split_whitespace();
            let bx = tokens.next().unwrap().parse::<i64>().unwrap();
            let by = tokens.next().unwrap().parse::<i64>().unwrap();

            let n = lines.next();
            let l = n.unwrap();
            let l = l.replace("Prize: X=", "");
            let l = l.replace(", Y=", " ");
            let mut tokens = l.split_whitespace();
            let px = tokens.next().unwrap().parse::<i64>().unwrap() +10000000000000i64;
            let py = tokens.next().unwrap().parse::<i64>().unwrap() +10000000000000i64;

            machines.push(Machine { ax, ay, bx, by, px, py });
        }

        Solution { machines }
    }

    fn solve(&self) -> i64 {
        let mut sum = 0;
        for m in &self.machines {
            let s = m.solve();
            sum += s;
        }
        sum
    }
}

fn main() {
    let time = time::Instant::now();

    let solution = Solution::from_file("input.txt");
    println!("solution: {}", solution.solve());
    println!("time: {:?}", time.elapsed());
}
