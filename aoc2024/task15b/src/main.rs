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
                if self.tiles[y as usize][x as usize] == '[' {
                    gps += y *100 + x;
                }
            }
        }
        gps
    }

    fn print(&self, robot: (i32, i32)) {
        for y in 0..self.height {
            for x in 0..self.width {
                if x == robot.0 && y == robot.1 {
                    print!("@");
                }
                else {
                    print!("{}", self.tiles[y as usize][x as usize]);
                }
            }
            println!();
        }
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
            width = line.len() as i32 * 2;
            height += 1;
            let mut row: Vec<char> = Vec::new();
            line.chars().for_each(|c| {
                match c {
                    '#' => {
                        row.push('#');
                        row.push('#');
                    },
                    'O' => {
                        row.push('[');
                        row.push(']');
                    },
                    _ => {
                        row.push(c);
                        row.push('.');
                    }
                }
            });
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

    fn push(&mut self, x:i32, y:i32, dx:i32, dy:i32, simulate: bool) -> bool {
        if dy == 0 {
            match self.map.tiles[y as usize][(x + dx * 2) as usize] {
                '.' => {
                    if !simulate {
                        self.map.tiles[y as usize][x as usize] = '.';
                        self.map.tiles[y as usize][(x + dx * 2) as usize] = if dx < 0 {'['} else {']'};
                        self.map.tiles[y as usize][(x + dx) as usize] = if dx < 0 {']'} else {'['};
                    }
                    return true;
                },
                '[' | ']' => {
                    let val = self.push(x + dx*2, y, dx, dy, simulate);
                    if !simulate && val {
                        self.map.tiles[y as usize][x as usize] = '.';
                        self.map.tiles[y as usize][(x + dx * 2) as usize] = if dx < 0 {'['} else {']'};
                        self.map.tiles[y as usize][(x + dx) as usize] = if dx < 0 {']'} else {'['};
                    }
                    return val;
                },
                _ => return false,
            }
        }
        else {
            match self.map.tiles[(y + dy) as usize][x as usize] {
                '.' => {
                    if !simulate {
                        self.map.tiles[(y + dy) as usize][x as usize] = self.map.tiles[y as usize][x as usize];
                        self.map.tiles[y as usize][x as usize] = '.';
                    }
                    return true;
                },
                '[' =>{
                    let val = self.push(x, y + dy, dx, dy, simulate) && self.push(x + 1, y + dy, dx, dy, simulate);
                    if val && !simulate {
                        self.map.tiles[(y + dy) as usize][x as usize] = self.map.tiles[y as usize][x as usize];
                        self.map.tiles[y as usize][x as usize] = '.';
                    }
                    return val;
                },
                ']' =>{
                    let val = self.push(x, y + dy, dx, dy, simulate) && self.push(x - 1, y + dy, dx, dy, simulate);
                    if val && !simulate {
                        self.map.tiles[(y + dy) as usize][x as usize] = self.map.tiles[y as usize][x as usize];
                        self.map.tiles[y as usize][x as usize] = '.';
                    }
                    return val;
               },
                _ => return false,
            }
        }
    }

    fn make_move(&mut self, dx:i32, dy:i32) {
        let (x, y) = self.robot;
        match self.map.tiles[(y + dy) as usize][(x + dx) as usize] {
            '.' => {
                self.robot = (x + dx, y + dy);
            },
            '[' => {
                if self.push(x + dx, y + dy, dx, dy, true) && (dy == 0 || self.push(x + 1, y + dy, dx, dy, true)) {
                    self.push(x + dx, y + dy, dx, dy, false);
                    if dy != 0 {
                        self.push(x + dx + 1, y + dy, dx, dy, false);
                    }
                    self.robot = (x + dx, y + dy);
                }
            },
            ']' => {
                if self.push(x + dx, y + dy, dx, dy, true) && (dy == 0 || self.push(x - 1, y + dy, dx, dy, true)) {
                    self.push(x + dx, y + dy, dx, dy, false);
                    if dy != 0 {
                        self.push(x - 1, y + dy, dx, dy, false);
                    }
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
            //println!("Move: {}", self.moves[i]);
            //self.map.print(self.robot);
        }
    }


}

fn main() {
    let mut solution = Solution::new("input.txt");
    solution.map.print(solution.robot);
    solution.simulate();
    solution.map.print(solution.robot);
    println!("{}", solution.map.calculate_gps());
}
