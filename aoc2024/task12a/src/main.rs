use std::time;
use std::fs;

struct Map {
    width: i32,
    height: i32,
    data: Vec<char>,
}

impl Map {
    fn new(width: i32, height: i32) -> Map {
        Map {
            width,
            height,
            data: vec![' '; (width * height) as usize],
        }
    }

    fn from_file(filename: &str) -> Map {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let width = contents.lines().next().unwrap().len() as i32;
        let height = contents.lines().count() as i32;
        let data = contents.chars().filter(|x| {*x!='\n'&&*x!='\r'}).collect();
        Map {
            width,
            height,
            data,
        }
    }

    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return ' ';
        }
        self.data[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, value: char) {
        self.data[(y * self.width + x) as usize] = value;
    }
}

struct Solution
{
    map: Map,
}

impl Solution {
    fn calculate_region(&self, x: i32, y: i32, map2: &mut Map) -> i64 {
        let mut area = 0i64;
        let mut perimeter = 0i64;
        let mut stack = Vec::new();
        let current = self.map.get(x, y);
        stack.push((x, y));
        while ! stack.is_empty() {
            let (x, y) = stack.pop().unwrap();
            if map2.get(x, y) == 'X' {
                continue;
            }
            //println!("({}, {})", x, y);
            map2.set(x, y, 'X');
            area += 1;
            if self.map.get(x+1,y) == current && map2.get(x+1,y) != 'X' {
                stack.push((x+1, y));
            }
            if self.map.get(x-1,y) == current && map2.get(x-1,y) != 'X' {
                stack.push((x-1, y));
            }
            if self.map.get(x,y+1) == current && map2.get(x,y+1) != 'X' {
                stack.push((x, y+1));
            }
            if self.map.get(x,y-1) == current && map2.get(x,y-1) != 'X' {
                stack.push((x, y-1));
            }
            if self.map.get(x+1,y) != current {
                perimeter += 1;
            }
            if self.map.get(x-1,y) != current {
                perimeter += 1;
            }
            if self.map.get(x,y+1) != current {
                perimeter += 1;
            }
            if self.map.get(x,y-1) != current {
                perimeter += 1;
            }
        }
        //println!("Field: {}, Area: {}, Perimeter: {}", current, area, perimeter);
        area * perimeter
    }

    fn calculate_fences(&self) -> i64 {
        let mut result = 0i64;
        let mut map2 = Map::new(self.map.width, self.map.height);
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if map2.get(x, y) != 'X' {
                    result += self.calculate_region(x, y, &mut map2);
                }
            }
        }
        result
    }
}

fn main() {
    let time = time::Instant::now();
    let solution = Solution {
        map: Map::from_file("input.txt"),
    };
    println!("Cost: {}", solution.calculate_fences());
    println!("Time: {}ms", time.elapsed().as_millis());
}
