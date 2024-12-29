use std::fs;

struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Vec<char>>,
}

struct Solution {
    map: Map,
    moves: Vec<char>,
    robot: (i32, i32),
}

impl Map {
    fn calculate_gps(&self) -> i32 {
        let mut gps = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y as usize][x as usize] == 'O' {
                    gps += y *100 + x;
                }
            }
        }
        gps
    }
}


impl Solution {
    fn new(file_name: &str) -> Solution {
        let content = fs::read_to_string(file_name).expect("Error reading file");
        let mut lines = content.lines();
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::new();
        let mut moves = Vec::new();
        let mut robot = (0, 0);
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() { break; }
            width = line.len() as i32;
            height += 1;
            let mut row: Vec<char> = line.chars().collect();
            for i in 0..width {
                if row[i as usize] == '@' {
                    robot = (i, height - 1);
                    row[i as usize] = '.';
                }
            }
            tiles.push(row);
        }

        loop {
            let line = lines.next();
            if line.is_none() { break; }
            line.unwrap().chars().for_each(|c| moves.push(c));
        }

        Solution {
            map: Map {
                width,
                height,
                tiles,
            },
            moves,
            robot
        }
    }

    fn push(&mut self, x:i32, y:i32, dx:i32, dy:i32) -> bool {

        let mut i = 1;
        loop {
            match self.map.tiles[(y + dy * i) as usize][(x + dx * i) as usize] {
                '#' => return false,
                '.' => {
                    self.map.tiles[(y + dy * i) as usize][(x + dx * i) as usize] = 'O';
                    self.map.tiles[y as usize][x as usize] = '.';
                    return true;
                },
                _ => {}
            }
            i+=1;
        }
    }

    fn make_move(&mut self, dx:i32, dy:i32) {
        let (x, y) = self.robot;
        match self.map.tiles[(y + dy) as usize][(x + dx) as usize] {
            '.' => {
                self.robot = (x + dx, y + dy);
            },
            'O' => {
                if self.push(x + dx, y + dy, dx, dy) {
                    self.robot = (x + dx, y + dy);
                }
            },
            _ => {}
        }
    }

    fn simulate(&mut self) {
        for i in 0..self.moves.len() {
            match self.moves[i] {
                '^' => self.make_move(0, -1),
                'v' => self.make_move(0, 1),
                '<' => self.make_move(-1, 0),
                '>' => self.make_move(1, 0),
                _ => {}
            }
        }
    }


}

fn main() {
    let mut solution = Solution::new("input.txt");
    solution.simulate();
    println!("{}", solution.map.calculate_gps());
}
