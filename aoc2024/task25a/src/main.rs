use std::fs;

#[derive(Debug)]
struct Solution {
    keys: Vec<Vec<i32>>,
    locks: Vec<Vec<i32>>,
}

impl Solution {
    fn new(file_name: &str) -> Solution {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
        let mut lines = contents.lines();
        loop {
            let line = lines.next();
            if line == None {
                break;
            }
            let line = line.unwrap();
            if line == "#####" { // lock
                let mut lock = vec![0,0,0,0,0];
                for i in 0..5{
                    let line = lines.next().unwrap();
                    for j in 0..5 {
                        if line.chars().nth(j).unwrap() == '#' {
                            lock[j] = i+1;
                        }
                    }
                }
                let line = lines.next().unwrap();
                assert!(line == ".....");
                locks.push(lock);
            }
            else if line == "....." { // key
                let mut key = vec![5,5,5,5,5];
                for i in 0..5{
                    let line = lines.next().unwrap();
                    for j in 0..5 {
                        if line.chars().nth(j).unwrap() == '.' {
                            key[j] = 4-i;
                        }
                    }
                }
                let line = lines.next().unwrap();
                assert!(line == "#####");
                keys.push(key);
            }
        }


        Self { keys, locks }
    }

    fn find_fits(&self) -> u64 {
        let mut fits = 0;
        for key in &self.keys {
            for lock in &self.locks {
                let mut fits_key = true;
                for i in 0..5 {
                    if key[i] + lock[i] > 5 {
                        fits_key = false;
                        break;
                    }
                }
                if fits_key {
                    fits += 1;
                }
            }
        }
        fits
    }

}
fn main() {
    let solution = Solution::new("input.txt");

    println!("{:?}", solution);

    println!("Fits: {}", solution.find_fits());
}
