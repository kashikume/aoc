
fn calculate(num: u64, steps: u64) -> u64 {
    let mut result = num;
    for _ in 0..steps {
        let next = result * 64u64;
        result = (result ^ next) % 16777216;
        let next = result / 32u64;
        result = (result ^ next) % 16777216;
        let next = result * 2048u64;
        result = (result ^ next) % 16777216;
    }
    result
}

fn main() {
    let mut result = 0u64;


 
    std::fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .for_each(|line| {
            let num:u64 = line.parse().unwrap();
            result += calculate(num, 2000);
        });

    println!("Result: {}", result)
}
