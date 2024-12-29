use std::time;
use std::fs;

#[derive(PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Map {
    width: i32,
    height: i32,
    data: Vec<char>,
    antenas: Vec<Vec<Position>>,
}

impl Map {
    fn from_file(filename: &str) -> Map {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let height = contents.lines().count() as i32;
        let width = contents.lines().next().unwrap().len() as i32;
        let data = contents.chars().filter(|x|{*x!='\n' && *x!='\r'}).collect();
        let antenas = vec![vec![]; 256];
        Map { width, height, data, antenas }
    }

    fn get(&self, x: i32, y: i32) -> char {
        self.data[(y * self.width + x) as usize]
    }

    fn find_antenas(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y);
                if c != '.' {
                    self.antenas[c as usize].push(Position { x, y });
                }
            }
        }
    }

    fn check_antinode(x: i32, y:i32, antena: &Vec<Position>) -> bool {
        for a in antena.iter() {
            let dx = x - a.x;
            let dy = y - a.y;
            if dx == 0 && dy == 0 { continue; }
            let pos = Position { x: x + 2*dx, y: y + 2*dy };
            if antena.contains(&pos) { return true; }
            let pos = Position { x: x - 2*dx, y: y - 2*dy };
            if antena.contains(&pos) { return true; }
        }
        false
    }

    fn is_antinode(&self, x: i32, y:i32) -> bool {
        for a in self.antenas.iter() {
            if a.len() > 1 {
                if Self::check_antinode(x, y, a) {
                    return true;
                }
            }
        }
        false
    }

    fn find_antinodes(&self) -> i32 {
        let mut antinodes = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_antinode(x, y) {
                    antinodes += 1;
                }
            }   
        }
        antinodes
    }
}

fn main() {
    let time = time::Instant::now();
    let mut map = Map::from_file("input.txt");
    map.find_antenas();
    println!("Antinodes: {:?}", map.find_antinodes());
    println!("Time: {:?}", time.elapsed());
}
