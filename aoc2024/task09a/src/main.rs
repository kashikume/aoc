use std::time;
use std::fs;

#[derive(PartialEq)]
enum Block {
    Free,
    File(i64),
}

struct Disc {
    blocks: Vec<Block>,
}

impl Disc {
    fn from_file(file_name: &str) -> Disc {
        let contents = fs::read_to_string(file_name).expect("Error reading file");
        let mut blocks = Vec::new();
        let mut is_file = true;
        let mut file_id: i64 = 0;
        for c in contents.chars() {
            let num = c.to_digit(10).unwrap() as i32;
            if is_file {
                for _ in 0..num {
                    blocks.push(Block::File(file_id));
                }
                file_id += 1;
            } else {
                for _ in 0..num {
                    blocks.push(Block::Free);
                }
            }
            is_file = !is_file;
        }
        Disc { blocks }
    }

    fn defragment(&mut self) {
        let mut last_file = self.blocks.len() - 1;
        for i in 0..self.blocks.len() {
            if self.blocks[i] == Block::Free {
                while(last_file > 0 && self.blocks[last_file] == Block::Free) {
                    last_file -= 1;
                }
                if last_file <= i {
                    break;
                }
                self.blocks.swap(i, last_file);
            }
        }
    }

    fn checksum(&self) -> i64 {
        let mut sum = 0;
        for (pos, block) in self.blocks.iter().enumerate() {
            match block {
                Block::File(id) => {
                    sum += id * pos as i64;
                }
                Block::Free => {}
            }
        }
        sum
    }
}

fn main() {
    let time = time::Instant::now();

    let mut disc = Disc::from_file("input.txt");
    disc.defragment();
    println!("Checksum: {}", disc.checksum());

    println!("Time elapsed: {:?}", time.elapsed());
}
