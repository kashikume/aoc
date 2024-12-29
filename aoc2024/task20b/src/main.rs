struct Map {
    width: u32,
    height: u32,
    tiles: Vec<char>,
    distance_to_end: Vec<usize>,
    distance_to_start: Vec<usize>,
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

        let distance_to_end = vec![usize::MAX; tiles.len()];
        let distance_to_start = vec![usize::MAX; tiles.len()];
        let path = Vec::new();

        Map {
            width,
            height,
            tiles,
            distance_to_end,
            distance_to_start,
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

    fn calc_distances(&mut self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.tiles.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        distances[start] = 0;
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let (x, y) = self.get_xy(current);
            let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
            for (nx, ny) in neighbors.iter() {
                if *nx < self.width && *ny < self.height {
                    let index = self.get_index(*nx, *ny);
                    if self.tiles[index] != '#' && distances[index] == usize::MAX {
                        distances[index] = distances[current] + 1;
                        queue.push_back(index);
                    }
                }
            }
        }
        distances
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
                if self.tiles[index] != '#' && self.distance_to_end[index] < best_dist {
                    best_dist = self.distance_to_end[index];
                    best_index = index;
                }
            }
            current = best_index;
            self.path.push(current);
        }
    }

    fn calc_wall_distances(&mut self) {
        let mut new_dist = self.distance_to_start.clone();
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
                        if self.distance_to_start[*neighbor] != usize::MAX {
                            best_dist = std::cmp::min(best_dist, self.distance_to_start[*neighbor] + (1 as usize));
                        }
                    }
                    new_dist[index] = best_dist;
                }
            }
        }
        self.distance_to_start = new_dist;
    }

    fn find_shortcuts(&self, size: i32, save: usize) -> usize {
        let mut found = 0;
        for i in self.path.iter() {
            let (x,y) = self.get_xy(*i);
            for dx in -size .. size {
                for dy in  -size .. size {
                    let dist = dx.abs() + dy.abs();
                    if dist > size || (dx == 0 && dy == 0) {
                        continue;
                    }
                    let x2 = x as i32 + dx;
                    let y2 = y as i32 + dy;
                    if x2 <= 0 || y2 <= 0 || x2 > self.width as i32 - 1 || y2 > self.height as i32 - 1 {
                        continue;
                    }

                    let end_index = self.get_index(x2 as u32, y2 as u32);
                    if self.tiles[end_index] == '#' {
                        continue;
                    }

                    if self.distance_to_end[end_index] < self.distance_to_end[*i] && (self.distance_to_end[*i] - self.distance_to_end[end_index]) + dist as usize >= save {
                        println!("({}, {}) -> ({}, {}) = {}, saved: {}", x, y, x2, y2, dist, (self.distance_to_end[*i] - self.distance_to_end[end_index]) + dist as usize);
                        found += 1;
                    }
                }
            }
        }
        found
    }

    fn find_shortcuts2(&self, size: usize, save: usize) -> usize{
        let mut found: usize = 0;
        for i1 in 0..self.distance_to_end.len() {
            if self.distance_to_start[i1] == usize::MAX {
                continue;
            }
            if self.tiles[i1] == '#' {
                continue;
            }
            for i2 in 0 .. self.distance_to_end.len() {
                if self.distance_to_end[i2] == usize::MAX {
                    continue;
                }
                if i1 == i2 {
                    continue;
                }
                if self.tiles[i2] == '#' {
                    continue;
                }
    
                let (x1, y1) = self.get_xy(i1);
                let (x2, y2) = self.get_xy(i2);
                let dist = ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize;
                
                if dist > size {
                    continue;
                }

                if self.distance_to_end[i1] < self.distance_to_end[i2] {
                    continue;
                }
                let new_dist = self.distance_to_start[i1] + dist + self.distance_to_end[i2];
                if new_dist >= self.path.len() {
                    continue;
                }
                let saving = self.path.len() - new_dist - 1;

                if saving >= save {
                    //println!("({}, {}) -> ({}, {}) = {}, saved: {}", x1, y1, x2, y2, dist, saving);
                    found+=1
                }
            }
        }
        found
    }
}

// to low: 897802
// to low: 988531


fn main() {
    let mut map = Map::new("input.txt");
    map.distance_to_end = map.calc_distances( map.end);
    map.distance_to_start = map.calc_distances(map.start);
    map.find_path();
    //map.calc_wall_distances();

    println!("Part 2: {}", map.find_shortcuts2(20, 100));
}
