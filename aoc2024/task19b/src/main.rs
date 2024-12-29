use std::fs;
use std::collections::HashMap;
use std::time;

#[derive(Debug)]
struct Solution {
    patterns: Vec<String>,
    by_letter: Vec<Vec<String>>,
}

impl Solution{
    fn new(file_name:&str) -> Self {
        let mut patterns = Vec::new();

        let content = fs::read_to_string(file_name).expect("Error reading file");
        let mut lines = content.lines();
        let first_line = lines.next().unwrap();
        let towels: Vec<String> = first_line.split(", ").map(|s| s.to_string()).collect();
        let _empty_line = lines.next().unwrap();

        for line in lines {
            patterns.push(line.to_string());
        }

        let mut by_letter = vec![Vec::new(); 'z' as usize - 'a' as usize + 1];
        for towel in towels.iter() {
            let first_letter = towel.chars().next().unwrap();
            by_letter[first_letter as usize - 'a' as usize].push(towel.clone());
        }

        Self { patterns, by_letter }
    }

    fn find_match(&self, pattern: &str, cache: &mut HashMap<String, u64>) -> u64 {
        if cache.contains_key(pattern) {
            return *cache.get(pattern).unwrap();
        }
        let mut result = 0u64;
        let first_letter = pattern.chars().next().unwrap() as usize - 'a' as usize;
        for towel in self.by_letter[first_letter].iter() {
            if pattern.starts_with(towel) {
                if pattern == towel {
                    result+=1;
                } else {
                    result += self.find_match(&pattern[towel.len()..], cache) 
                }
            }
        }
        cache.insert(pattern.to_string(), result);
        result
    }
}

fn main() {
    let start = time::Instant::now();
    let solution = Solution::new("input.txt");
    //println!("{:?}",solution);
    let mut result = 0u64;
    let mut cache: HashMap<String, u64> = HashMap::new();
    for pattern in solution.patterns.iter() {
        let sol = solution.find_match(&pattern, &mut cache);
        result += sol;
        //println!("{} -> {}", pattern, sol);
    }
    println!("{:?}", result);
    println!("Elapsed time: {:?}", start.elapsed());
}
