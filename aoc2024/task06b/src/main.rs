use std::fs;

#[derive(Debug, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Map {
    width: i32,
    height: i32,
    tiles: Vec<char>,
}

struct Direction {
    dx: i32,
    dy: i32,
}

impl Map {
    fn new(file_name: &str) -> Map {
        let content = fs::read_to_string(file_name).expect("Could not read file");
        let height = content.lines().count() as i32;
        let width = content.lines().next().unwrap().chars().count() as i32;
        let tiles = content
            .chars()
            .filter(|c| *c != '\n' && *c != '\r')
            .collect::<Vec<char>>();
        Map { width, height, tiles }
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }

    fn get_tile(&self, x: i32, y: i32) -> char {
        match self.get_index(x, y)
        {
            Some(index) => self.tiles[index],
            None => ' ',
        }
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: char) {
        match self.get_index(x, y){
            Some(index) => self.tiles[index] = tile,
            None => (),
        }
    }

    fn find_start(&self) -> Pos {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_tile(x, y) == '^' {
                    return Pos { x, y };
                }
            }
        }
        Pos { x: -1, y: -1 }
    }

    fn go_forward(&self, mut pos: Pos, dir: &Direction) -> Option<Pos> {
        loop {
            let new_pos = Pos{x: pos.x + dir.dx, y: pos.y + dir.dy};
            match self.get_tile(new_pos.x, new_pos.y) {
                '#' => {
                    return Some(pos);
                }
                ' ' => {
                    return None;
                }
                _ => {
                    pos = new_pos;
                }
            }

        }
    }

    fn dir_pos_value(&self, pos: &Pos, dir: i32) -> i32 {
        pos.x + pos.y * self.width + dir * self.width * self.height
    }

    fn check_for_loop(&self, mut pos: Pos) -> bool {
        let directions = [
            Direction { dx: 0, dy: -1 },
            Direction { dx: 1, dy: 0 },
            Direction { dx: 0, dy: 1 },
            Direction { dx: -1, dy: 0 },
        ];
        let mut dir = 0;
        let mut vis = Vec::new();
        loop {
            pos = match self.go_forward(pos.clone(), &directions[dir]) {
                Some(p) => {
                    let v = self.dir_pos_value(&p, dir as i32);
                    if vis.contains(&v) {
                        return true;
                    }
                    vis.push(v);
                    dir = (dir + 1) % 4;
                    p
                }
                None => return false,
            };
        }
    }   
}

fn main() {
    let time = std::time::Instant::now();
    let mut map = Map::new("input.txt");
    let start = map.find_start();
    let mut result = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get_tile(x, y) == '.' {
                map.set_tile(x, y, '#');
                if map.check_for_loop(start.clone()) {
                        result += 1;
                }
                map.set_tile(x, y, '.');
            } 
        }
    }
    println!("Result: {}", result);
    println!("{:.2} ms", time.elapsed().as_millis());
}
