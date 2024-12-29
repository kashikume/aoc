use std::cmp::min;
use std::time;
use std::fs;

// ax + bx = px
// ay + by = py


struct Machine {
    ax: i32,
    ay: i32,
    bx: i32,
    by: i32,
    px: i32,
    py: i32,
}

struct Solution {
    machines : Vec<Machine>,
}

impl Machine {
    fn solve(&self) -> i32 {
        let bx = self.px / self.bx as i32;
        let by = self.py / self.by as i32;
        let start_b = min(min(bx, by), 100);

        for b in (0..start_b+1).rev() {
            let bx = b * self.bx;
            let by = b * self.by;

            if (self.px - bx) % self.ax != 0 || (self.py - by) % self.ay != 0 {
                continue;
            }

            let ax = (self.px - bx) / self.ax;
            let ay = (self.py - by) / self.ay;
            if ax != ay { continue; }

            return ax * 3 + b;
        }
        0
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
            let ax = tokens.next().unwrap().parse::<i32>().unwrap();
            let ay = tokens.next().unwrap().parse::<i32>().unwrap();

            let n = lines.next();
            let l = n.unwrap();
            let l = l.replace("Button B: X+", "");
            let l = l.replace(", Y+", " ");
            let mut tokens = l.split_whitespace();
            let bx = tokens.next().unwrap().parse::<i32>().unwrap();
            let by = tokens.next().unwrap().parse::<i32>().unwrap();

            let n = lines.next();
            let l = n.unwrap();
            let l = l.replace("Prize: X=", "");
            let l = l.replace(", Y=", " ");
            let mut tokens = l.split_whitespace();
            let px = tokens.next().unwrap().parse::<i32>().unwrap();
            let py = tokens.next().unwrap().parse::<i32>().unwrap();

            machines.push(Machine { ax, ay, bx, by, px, py });
        }

        Solution { machines }
    }

    fn solve(&self) -> i32 {
        let mut sum = 0;
        for m in &self.machines {
            sum += m.solve();
        }
        sum
    }
}

fn main() {
    let time = time::Instant::now();

    let solution = Solution::from_file("input.txt");
    println!("solution: {}", solution.solve());
    println!("time: {}", time.elapsed().as_millis());
}
