use std::time;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Clone, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

struct Map {
    width: i32,
    height: i32,
    data: Vec<char>,
    antenas: Vec<Vec<Position>>,
    antinodes: HashSet<Position>,
}

impl Map {
    fn from_file(filename: &str) -> Map {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let height = contents.lines().count() as i32;
        let width = contents.lines().next().unwrap().len() as i32;
        let data = contents.chars().filter(|x|{*x!='\n' && *x!='\r'}).collect();
        let antenas = vec![vec![]; 256];
        Map { width, height, data, antenas, antinodes: HashSet::new() }
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

    fn find_antinodes(&mut self) -> i32 {
        for a in self.antenas.iter() {
            if a.len() < 2 { continue; }
            for i1 in 0..a.len()-1 {
                for i2 in i1+1..a.len() {
                    let dx = a[i1].x - a[i2].x;
                    let dy = a[i1].y - a[i2].y;

                    let mut f = 0;
                    loop {
                        let pos = Position { x: a[i1].x + f*dx, y: a[i1].y + f*dy };
                        if pos.x < 0 || pos.x >= self.width || pos.y < 0 || pos.y >= self.height {
                            break;
                        }
                        self.antinodes.insert(pos);
                        f+=1;
                    }
                    let mut f = 0;
                    loop {
                        let pos = Position { x: a[i1].x - f*dx, y: a[i1].y - f*dy };
                        if pos.x < 0 || pos.x >= self.width || pos.y < 0 || pos.y >= self.height {
                            break;
                        }
                        self.antinodes.insert(pos);
                        f+=1;
                    }
                }
            }
        }
        self.antinodes.len() as i32
    }
}

fn main() {
    let time = time::Instant::now();
    let mut map = Map::from_file("input.txt");
    map.find_antenas();
    println!("Antinodes: {:?}", map.find_antinodes());
    println!("Time: {:?}", time.elapsed());
}
