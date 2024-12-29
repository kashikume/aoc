use std::fs;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

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
        if(x < 0 || y < 0 || x >= self.width || y >= self.height) {
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

    fn go_forward(&mut self, mut pos: Pos, dir: &Direction) -> Option<Pos> {
        loop {
            let new_pos = Pos{x: pos.x + dir.dx, y: pos.y + dir.dy};
            self.set_tile(pos.x, pos.y, 'X');
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

    fn calculate(&self) -> i32 {
        let mut count = 0;
        for c in &self.tiles {
            if *c == 'X' {
                count += 1;
            }
        }
        count
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_tile(x, y));
            }
            println!();
        }
    }
}

fn main() {
    let mut map = Map::new("input.txt");
    let mut pos = map.find_start();

    let directions = [
        Direction { dx: 0, dy: -1 },
        Direction { dx: 1, dy: 0 },
        Direction { dx: 0, dy: 1 },
        Direction { dx: -1, dy: 0 },
    ];
    let mut dir = 0;
    loop {
        pos = match map.go_forward(pos, &directions[dir]) {
            Some(p) => {
                dir = (dir + 1) % 4;
                p
            }
            None => break,
        };
        //map.print();
        println!("Result: {} {:?}", map.calculate(), pos);
    }
    //map.print();
    println!("Result: {}", map.calculate());
}
