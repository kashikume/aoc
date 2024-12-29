use std::time;
use std::fs;

struct Robot{
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

struct Solution {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

impl Solution {
    fn new(file_name: &str, width: i64, height: i64) -> Solution {
        let contents = fs::read_to_string(file_name)
            .expect("Something went wrong reading the file");

        let mut robots = Vec::new();
        for line in contents.lines() {
            let l2 = line.replace("p=", "")
            .replace("v=", "")
            .replace(",", " ");

            let parts: Vec<&str> = l2.split(" ").collect();
            let px = parts[0].parse::<i64>().unwrap();
            let py = parts[1].parse::<i64>().unwrap();
            let vx = parts[2].parse::<i64>().unwrap();
            let vy = parts[3].parse::<i64>().unwrap();
            robots.push(Robot{px, py, vx, vy});
        }

        Solution{robots, width, height}
    }

    fn simulate(&mut self, seconds: i64) {
        for robot in &mut self.robots {
            robot.px += robot.vx * seconds;
            while(robot.px < 0) {
                robot.px = self.width + robot.px;
            }
            robot.px %= self.width;
            robot.py += robot.vy * seconds;
            while(robot.py < 0) {
                robot.py = self.height + robot.py;
            }
            robot.py %= self.height;
        }
    }

    fn calculate_safe_area(&self) -> i64 {
        let mut tl = 0;
        let mut tr = 0;
        let mut bl = 0;
        let mut br = 0;

        for robot in &self.robots {
            if robot.px < self.width / 2 && robot.py < self.height / 2 {
                tl += 1;
            } else if robot.px > self.width / 2 && robot.py < self.height / 2 {
                tr += 1;
            } else if robot.px < self.width / 2 && robot.py > self.height / 2 {
                bl += 1;
            } else if robot.px > self.width / 2 && robot.py > self.height / 2 {
                br += 1;
            }
            println!("px: {}, py: {}", robot.px, robot.py);
        }
        println!("tl: {}, tr: {}, bl: {}, br: {}", tl, tr, bl, br);
        tr * tl * br * bl
    }
}


fn main() {
    let time = time::SystemTime::now();

    let mut solution = Solution::new("input.txt", 101, 103);
    solution.simulate(100);
    println!("{}", solution.calculate_safe_area());
    println!("{:?}", time.elapsed().unwrap());
}
