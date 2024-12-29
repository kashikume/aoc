use std::{cmp::min, fs, time};

// ^:0 >:1  v:2  <:3

struct Map {
    width: usize,
    height: usize,
    data: Vec<char>,
    start: usize,
    end: usize,
    ins: Vec<Vec<usize>>,
}

struct Pos {
    i: usize,
    rot: usize,
    dist: usize,
}

fn turn_cost(from: usize, to: usize) -> usize {
    if from == to {
        0
    } else if (from + 2) % 4 == to {
        2000
    } else {
        1000
    }
}

fn dx_dy(rot: usize) -> (i32, i32) {
    match rot {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => (0, 0),
    }
}

impl Map {
    fn new(file_name: &str) -> Self {
        let contents = fs::read_to_string(file_name).unwrap();
        let mut width = 0;
        let mut height = 0;
        let mut start = 0;
        let mut end = 0;
        let mut data = Vec::new();
        for (i, line) in contents.lines().enumerate() {
            width = line.len();
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => start = i * width + j,
                    'E' => end = i * width + j,
                    _ => (),
                }
                data.push(c);
            }
            height += 1;
        }

        let ins = vec![vec![usize::MAX/2; 4]; width * height];

        Map {
            width,
            height,
            data,
            start,
            end,
            ins,
        }
    }

    fn xy_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_xy(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn calc_distances(&mut self) {
        let mut stack = Vec::new();
        stack.push(Pos { i: self.start, rot: 1, dist: 0 });
        stack.push(Pos { i: self.start, rot: 0, dist: 1000 });
        stack.push(Pos { i: self.start, rot: 2, dist: 1000 });
        stack.push(Pos { i: self.start, rot: 3, dist: 2000 });

        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            if self.data[pos.i] == '#' { continue; }
            if self.ins[pos.i][pos.rot] <= pos.dist { continue; }
            self.ins[pos.i][pos.rot] = pos.dist;

            for r in 0..4 {
                let cost = turn_cost(pos.rot, r);
                let (dx, dy) = dx_dy(r);
                let (x, y) = self.index_to_xy(pos.i);
                let i2 = self.xy_to_index((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                stack.push(Pos { i: i2, rot: r, dist: pos.dist + cost + 1 });
            }
        }
    }

    fn result1(&self) -> usize{
        let mut min_dist = usize::MAX;
        for i in 0..4 {
            min_dist = min_dist.min(self.ins[self.end][i]);
        }
        min_dist
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[self.xy_to_index(x, y)]);
            }
            println!();
        }
    }

    fn find_all_best(&mut self) {
        let mut stack = Vec::new();
        let best = self.result1();
        for i in 0..self.ins[self.end].len() {
            if self.ins[self.end][i] == best {
                stack.push(Pos { i: self.end, rot: i, dist: 0 });
            }
        }

        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            self.data[pos.i] = 'O';
            if pos.i == self.start { continue; }
            let mut best = usize::MAX;
            for i in 0..4 {
                let c = turn_cost(pos.rot, i) + self.ins[pos.i][i];
                best = min(c, best);
            }
            let (x, y) = self.index_to_xy(pos.i);
            for i in 0..4 {
                if self.ins[pos.i][i] + turn_cost(pos.rot, i) == best {
                    let (dx, dy) = dx_dy(i);
                    stack.push(Pos { i: self.xy_to_index((x as i32 - dx) as usize, (y as i32 - dy) as usize), rot: i, dist: best });
                }
            }
        }
    }

    fn result2(&self) -> usize {
        self.data.iter().filter(|&c| *c == 'O').count()
    }
}

fn main() {
    let timer = time::Instant::now();
    let mut map = Map::new("input.txt");
    map.calc_distances();
    println!("Part1: {}", map.result1());
    map.print();
    map.find_all_best();
    map.print();
    println!("Part2: {}", map.result2());
    println!("Elapsed: {}ms", timer.elapsed().as_millis());
}
