use std::time;
use std::fs;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Value {
    value: u64,
    iter: i32,
}

struct Solution {
    cache: HashMap<Value, u64>,
}

impl Solution {
    fn new() -> Solution {
        Solution {
            cache: HashMap::new(),
        }
    }
    fn cache_calc_stone(&mut self, stone: u64, iter: i32) -> u64 {
        let value = Value { value: stone, iter: iter };
        if self.cache.contains_key(&value) {
            return self.cache[&value];
        }
        let result = self.calc_stone(stone, iter);
        self.cache.insert(value, result);
        return result;
    }

    fn calc_stone(&mut self, stone: u64, iter: i32) -> u64 {
        if iter == 0 { return 1; }
        if stone == 0 { return self.cache_calc_stone(1, iter-1); }
        let sstone = format!("{}",stone);
        if sstone.len() % 2 == 0 {
            return self.cache_calc_stone(sstone[0..sstone.len()/2].parse::<u64>().unwrap(), iter-1) 
            + self.cache_calc_stone(sstone[sstone.len()/2..].parse::<u64>().unwrap(), iter-1);
        }
        else {
            return self.cache_calc_stone(stone * 2024, iter-1);
        }
    }
}


fn main() {
    let time = time::Instant::now();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let stones = contents.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    let mut solution = Solution::new();

    let mut result = 0u64;
    for stone in stones.iter() {
        println!("Calculating stone: {}", stone);

        result += solution.cache_calc_stone(*stone, 75);
    }


    
    println!("Result: {}", result);

    println!("{:?}", time.elapsed());
}
