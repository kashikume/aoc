use std::fs;
use std::time;
use std::collections::HashSet;

struct Map {
    width: i32,
    height: i32,
    data: Vec<i32>,
}

impl Map {
    fn from_file(file_name: &str) -> Map {
        let content = fs::read_to_string(file_name).expect("Failed to read file");
        let width = content.lines().next().unwrap().len() as i32;
        let height = content.lines().count() as i32;
        let data = content.chars()
            .filter(|&c| c != '\n' && c != '\r')
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        Map{width, height, data}
    }

    fn pos_to_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn index_to_pos(&self, index: usize) -> (i32, i32) {
        let x = index as i32 % self.width;
        let y = index as i32 / self.width;
        (x, y)
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return -1;
        }
        self.data[self.pos_to_index(x, y)]
    }

    fn calculate_score_for(&self, index: usize) -> i32 {
        let mut check = HashSet::new();
        check.insert(index);
        for value in 1..10 {
            let mut next_check = HashSet::new();
            for index in check.iter() {
                let (x, y) = self.index_to_pos(*index);
                if self.get(x - 1, y) == value {
                    next_check.insert(self.pos_to_index(x - 1, y));
                }
                if self.get(x + 1, y) == value {
                    next_check.insert(self.pos_to_index(x + 1, y));
                }
                if self.get(x, y - 1) == value {
                    next_check.insert(self.pos_to_index(x, y - 1));
                }
                if self.get(x, y + 1) == value {
                    next_check.insert(self.pos_to_index(x, y + 1));
                }
            }
            check = next_check
        }
        check.len() as i32
    }

    fn calculate_score(&self) -> i32 {
        let mut score = 0;
        for (i, v) in self.data.iter().enumerate() {
            if *v == 0 {
                score += self.calculate_score_for(i);
            }
        }
        score
    }
}

fn main() {
    let time = time::Instant::now();
    println!("Score: {}", Map::from_file("input.txt").calculate_score());
    println!("{:?}", time.elapsed());
}
