use std::fs;

#[derive(Debug)]
struct Solution {
    towels: Vec<String>,
    patterns: Vec<String>,
}

impl Solution{
    fn new(file_name:&str) -> Self {
        let mut patterns = Vec::new();

        let mut content = fs::read_to_string(file_name).expect("Error reading file");
        let mut lines = content.lines();
        let first_line = lines.next().unwrap();
        let mut towels = first_line.split(", ").map(|s| s.to_string()).collect();
        let _empty_line = lines.next().unwrap();

        for line in lines {
            patterns.push(line.to_string());
        }
        
        Self { towels, patterns }
    }

    fn find_match(&self, pattern: &str) -> bool {
        for towel in self.towels.iter() {
            if pattern.starts_with(towel) {
                if pattern == towel {
                    return true;
                } else {
                    if self.find_match(&pattern[towel.len()..]) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    let solution = Solution::new("input.txt");
    let mut result = 0;
    for pattern in solution.patterns.iter() {
        let sol = solution.find_match(pattern);
        result += if sol { 1 } else { 0 };
        println!("{} -> {}", pattern, sol);
    }
    println!("{:?}", result);
}
