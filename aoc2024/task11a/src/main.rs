use std::time;
use std::fs;

fn main() {
    let time = time::Instant::now();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let mut stones = contents.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        for s in stones {
            if s == 0 { new_stones.push(1); }
            else {
                let sstone = format!("{}",s);
                if sstone.len() % 2 == 0 {
                    new_stones.push(sstone[0..sstone.len()/2].parse::<u64>().unwrap());
                    new_stones.push(sstone[sstone.len()/2..].parse::<u64>().unwrap());
                }
                else {
                    new_stones.push(s * 2024);
                }
            }
        }
        stones = new_stones;
    }
    println!("Result: {}", stones.len());

    println!("{:?}", time.elapsed());
}
