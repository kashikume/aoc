struct Map {
    width: u32,
    height: u32,
    tiles: Vec<char>,
    distance: Vec<usize>,
    start: usize,
    end: usize,
    path: Vec<usize>,
}

impl Map {
    fn new(file_name: &str) -> Map {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::new();

        let mut start = 0;
        let mut end = 0;

        let file = std::fs::read_to_string(file_name).expect("Failed to read file");
        for line in file.lines() {
            height += 1;
            width = line.len() as u32;
            for c in line.chars() {
                if c == 'S' {
                    start = tiles.len();
                } else if c == 'E' {
                    end = tiles.len();
                }
                tiles.push(c);
            }
        }

        let distance = vec![usize::MAX; tiles.len()];
        let path = Vec::new();

        Map {
            width,
            height,
            tiles,
            distance,
            start,
            end,
            path
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn get_xy(&self, index: usize) -> (u32, u32) {
        let x = index % self.width as usize;
        let y = index / self.width as usize;
        (x as u32, y as u32)
    }

    fn calc_distances(&mut self) {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(self.end);
        self.distance[self.end] = 0;
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let (x, y) = self.get_xy(current);
            let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
            for (nx, ny) in neighbors.iter() {
                if *nx < self.width && *ny < self.height {
                    let index = self.get_index(*nx, *ny);
                    if self.tiles[index] != '#' && self.distance[index] == usize::MAX {
                        self.distance[index] = self.distance[current] + 1;
                        queue.push_back(index);
                    }
                }
            }
        }
    }

    fn find_path(&mut self) {
        let mut current = self.start;
        self.path.push(current);
        while current != self.end {
            let (x, y) = self.get_xy(current);
            let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
            let mut best_dist = usize::MAX;
            let mut best_index = 0;
            for (nx, ny) in neighbors.iter() {
                let index = self.get_index(*nx, *ny);
                if self.tiles[index] != '#' && self.distance[index] < best_dist {
                    best_dist = self.distance[index];
                    best_index = index;
                }
            }
            current = best_index;
            self.path.push(current);
        }
    }

    fn calc_wall_distances(&mut self) {
        let mut new_dist = self.distance.clone();
        for y in 1..self.height-1 {
            for x in 1..self.width-1 {
                let index = self.get_index(x, y);
                if self.tiles[index] == '#' {
                    let neighbors = [
                        self.get_index(x + 1, y),
                        self.get_index(x, y + 1),
                        self.get_index(x - 1, y),
                        self.get_index(x, y - 1),
                    ];
                    let mut best_dist = usize::MAX;
                    for neighbor in neighbors.iter() {
                        if self.distance[*neighbor] != usize::MAX {
                            best_dist = std::cmp::min(best_dist, self.distance[*neighbor] + (1 as usize));
                        }
                    }
                    new_dist[index] = best_dist;
                }
            }
        }
        self.distance = new_dist;
    }

    fn find_shortcuts(&self, save: usize) -> usize {
        let mut found = 0;
        for i in self.path.iter() {
            let (x,y) = self.get_xy(*i);
            let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
            for (nx, ny) in neighbors.iter() {
                let index = self.get_index(*nx, *ny);
                if self.distance[index] < self.distance[*i] &&  self.distance[*i] - self.distance[index] > save {
                    found += 1;
                    println!("{} {} {}", nx, ny, self.distance[*i] - self.distance[index]-1);
                }
            }
        }
        found
    }

}

fn main() {
    let mut map = Map::new("input.txt");
    map.calc_distances();
    map.find_path();
    map.calc_wall_distances();
    println!("Part 1: {}", map.find_shortcuts(100));
}
