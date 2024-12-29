use std::time;
use std::fs;

#[derive(PartialEq)]
struct File {
    id: i64,
    blocks: i32,
}

#[derive(PartialEq)]
enum Block {
    Free(i32),
    File(File),
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
                    blocks.push(Block::File(File{id:file_id, blocks:num}));
                }
                file_id += 1;
            } else {
                for i in 0..num {
                    blocks.push(Block::Free(num-i));
                }
            }
            is_file = !is_file;
        }
        Disc { blocks }
    }

    fn find_space(&self, size: i32) -> Option<usize> {
        for (pos, block) in self.blocks.iter().enumerate() {
            match block {
                Block::Free(free) => {
                    if *free >= size {
                        return Some(pos);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn calculate_free_space(&mut self) {
        let mut space = 0;
        for pos in (0..self.blocks.len()).rev() {
            match self.blocks[pos] {
                Block::Free(_) => {
                    space += 1;
                    self.blocks[pos] = Block::Free(space);
                }
                _ => { space = 0;}
            }
        }
    }

    fn defragment(&mut self) {
        for pos in (0..self.blocks.len()).rev() {
            match self.blocks[pos] {
                Block::File(ref file) => {
                    match self.find_space(file.blocks) {
                        Some(new_pos) if new_pos < pos => {
                            for i in 0..file.blocks {
                                self.blocks.swap(pos - i as usize, new_pos + i as usize);
                            }
                            self.calculate_free_space();
                        }
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    }

    fn checksum(&self) -> i64 {
        let mut sum = 0;
        for (pos, block) in self.blocks.iter().enumerate() {
            match block {
                Block::File(file) => {
                    sum += file.id * pos as i64;
                }
                Block::Free(_) => {}
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
