use std::{collections::VecDeque, fs};

struct Map {
    width: i64,
    height: i64,
    data: Vec<char>,
    bytes: Vec<(i64,i64)>,
}

impl Map {
    fn new(file_name:&str, width:i64, height:i64) -> Map {
        let content = fs::read_to_string(file_name).expect("Error reading file");
        let mut bytes = Vec::new();
        for l in content.lines() {
            let mut parts = l.split(",");
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            bytes.push((x,y));
        } 

        let data = vec!['.'; (width*height) as usize];
        Map { width, height, data, bytes }
    }

    fn xy_to_index(&self, x:i64, y:i64) -> usize {
        (y * self.width + x) as usize
    }

    fn index_to_xy(&self, index:usize) -> (i64,i64) {
        (index as i64 % self.width, index as i64 / self.width)
    }

    fn fall(&mut self, num:usize) {
        //for i in 0..num {
            let (x,y) = self.bytes[num];
            let ind = self.xy_to_index(x,y);
            self.data[ind] = '#';
        //}
    }

    fn get(&self, x:i64, y:i64) -> char {
        if x<0 || x>=self.width || y<0 || y>=self.height {
            return '#';
        }
        self.data[self.xy_to_index(x,y)]
    }

    fn path(&mut self, x1:i64, y1:i64, x2:i64, y2:i64) -> i64 {
        let mut dist = vec![i64::MAX; (self.width*self.height) as usize];
        let start = self.xy_to_index(x1,y1);
        let end = self.xy_to_index(x2,y2);
        let mut queue = VecDeque::new();
        queue.push_back(start);
        dist[start] = 0;
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let (x,y) = self.index_to_xy(current);
            let current_dist = dist[current];
            if current == end {
                return current_dist;
            }
            for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)].iter() {
                let new_x = x + dx;
                let new_y = y + dy;
                if self.get(new_x,new_y) == '.' {
                    let new_index = self.xy_to_index(new_x,new_y);
                    if dist[new_index] > current_dist + 1 {
                        dist[new_index] = current_dist + 1;
                        queue.push_back(new_index);
                    }
                }
            }
        }
        return i64::MAX;
    }

}

fn main() {
   let mut map = Map::new("input.txt", 71, 71);
   for i in 0..map.bytes.len() {
        map.fall(i);
        let p = map.path(0,0,70,70);
        if p == i64::MAX {
            println!("No path found: {},{}", map.bytes[i].0, map.bytes[i].1);
            break;
        } 
   }

}
