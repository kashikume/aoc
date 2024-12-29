use std::collections::{HashMap, HashSet};

fn calculate_prices(num: u64, steps: u64) -> Vec<i32> {
    let mut result = num;
    let mut results = Vec::new();
    for _ in 0..steps {
        let next = result * 64u64;
        result = (result ^ next) % 16777216;
        let next = result / 32u64;
        result = (result ^ next) % 16777216;
        let next = result * 2048u64;
        result = (result ^ next) % 16777216;
        results.push((result%10) as i32);
    }
    results
}

fn calculate_distances(num: u64, prices: &Vec<i32>) -> Vec<i32> {
    let mut last = (num%10) as i32;
    let mut result = Vec::new();
    for p in prices.iter() {
        result.push(p-last);
        last = *p;
    }
    result
}

fn calculate_changes(prices: &Vec<i32>, distances: &Vec<i32>) -> HashMap<(i32, i32, i32, i32), i32> {
    let mut result = HashMap::new();
    for i in 3..distances.len() {
        let key = (distances[i-3], distances[i-2], distances[i-1], distances[i-0]);
        let value = prices[i];
        if !result.contains_key(&key) {
            result.insert(key, value);
        }
    }
    result
}

#[derive(Debug)]
struct Buyer {
    changes: HashMap<(i32, i32, i32, i32), i32>,
}

impl Buyer {
    fn new(num: u64) -> Buyer {
        let prices = calculate_prices(num, 2000);
        let distances = calculate_distances(num, &prices);
        let changes = calculate_changes(&prices, &distances);
        Buyer { changes }
    }
    
}

fn main() {
    let mut buyers = Vec::new();
 
    std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .for_each(|line| {
            let num:u64 = line.parse().unwrap();
            let buyer = Buyer::new(num) ;
            buyers.push(buyer); 
        });

    let mut to_check = HashSet::new();
    for b in buyers.iter() {
        for (key, _) in b.changes.iter() {
            to_check.insert(key);
        }
    }

    let mut best = 0;
    for c in to_check.iter() {
        let mut sum = 0;
        for b in buyers.iter() {
            if b.changes.contains_key(c) {
                sum += b.changes.get(c).unwrap();
            }
        }
        if sum > best {
            println!("New best: {:?} = {}", c, sum);
            best = sum;
        }
    }

    println!("Result: {}", best)
}
