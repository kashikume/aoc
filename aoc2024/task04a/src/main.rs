use std::fs;

struct Solution {
    data : Vec<char>,
    width : i32,
    height : i32,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn load(&mut self, filename: &str) {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let mut lines = contents.lines();
        let first_line = lines.next().unwrap();
        self.width = first_line.len() as i32;
        self.height = contents.lines().count() as i32;
        self.data = contents.chars().filter(|x| {*x != '\n' && *x != '\r'}).collect();
    }

    fn get(&self, x: i32, y: i32) -> char {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return ' ';
        }
        self.data[(y * self.width + x) as usize]
    }

    fn check(&self, word: &String, start_x: i32, start_y: i32, dx: i32, dy: i32) -> bool {
        let mut x = start_x;
        let mut y = start_y; 
        for c in word.chars() {
            if c != self.get(x, y) {
                return false;
            }
            x += dx;
            y += dy;
        }
        return true;
    }

    fn check_all(&self, word: &String) -> i32 {
        let mut out = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                for dx in -1..2{
                    for dy in -1..2 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if self.check(word, x, y, dx, dy) {
                            out += 1;
                        }
                    }
                }
            }
        }
        return out;
    }
}

fn main() {
    let mut solution = Solution::new();
    solution.load("input.txt");
    let count = solution.check_all(&String::from("XMAS"));
    println!("{}", count);
}
