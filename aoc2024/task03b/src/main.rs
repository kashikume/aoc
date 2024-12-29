use std::fs;

fn mul(input: &str, start:usize) -> i32
{
    if input[start..start+4] != *"mul(" { return 0 }
    let mut pos = start + 4;

    let mut dig1 = 0;
    let mut dig2 = 0;

    for p in 0..3 {
        let c = input.chars().nth(pos).unwrap();
        if c >= '0' && c <= '9' {
            dig1 = dig1 * 10 + (c as i32 - '0' as i32);
            pos +=1;
        } else {
            if p == 0 { return 0 }   
            else { break }
        }
    }
    if input.chars().nth(pos).unwrap() != ',' { return 0 }
    pos += 1;
    for p in 0..3 {
        let c = input.chars().nth(pos).unwrap();
        if c >= '0' && c <= '9' {
            dig2 = dig2 * 10 + (c as i32 - '0' as i32);
            pos +=1;
        } else {
            if p == 0 { return 0 }   
            else { break }
        }
    }
    if input.chars().nth(pos).unwrap() != ')' { return 0 }
    return dig1 * dig2;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut sum = 0;
    let mut enable = 1;
    for i in 0..input.len()-8 {
        sum += enable * mul(&input, i);
        if input[i..i+4] == *"do()" { enable = 1; }
        if input[i..i+7] == *"don't()" { enable = 0; }
    }
    println!("Sum: {}", sum);
}
