use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Keyboard {
    keys: HashMap<char, (i32, i32)>,
    emptys: Vec<(i32, i32)>,
}

impl Keyboard {
    fn as_numeric() -> Keyboard {
        Keyboard {
            keys: [
                ('1', (0, 2)),
                ('2', (1, 2)),
                ('3', (2, 2)),
                ('4', (0, 1)),
                ('5', (1, 1)),
                ('6', (2, 1)),
                ('7', (0, 0)),
                ('8', (1, 0)),
                ('9', (2, 0)),
                ('0', (1, 3)),
                ('A', (2, 3)),
            ]
            .iter()
            .cloned()
            .collect(),
            emptys: [(0, 3)].to_vec(),
        }
    }

    fn as_controll() -> Keyboard {
        Keyboard {
            keys: [
                ('^', (1, 0)),
                ('A', (2, 0)),
                ('<', (0, 1)),
                ('v', (1, 1)),
                ('>', (2, 1)),
            ]
            .iter()
            .cloned()
            .collect(),
            emptys: [(0, 0)].to_vec(),
        }
    }
}

struct Cache {
    cache: HashMap<(char, char, u64), u64>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            cache: HashMap::new(),
        }
    }

    fn get(&self, from: char, to: char, level: u64) -> Option<u64> {
        self.cache.get(&(from, to, level)).cloned()
    }

    fn set(&mut self, from: char, to: char, level: u64, value: u64) {
        self.cache.insert((from, to, level), value);
    }
}

fn generate_moves(keyboard: &Keyboard, start: char, end: char) -> Vec<String> {
    let mut moves = Vec::new();

    let start_pos = keyboard.keys.get(&start).unwrap();
    let end_pos = keyboard.keys.get(&end).unwrap();
    
    for vert in 0..2 {
        let mut curr_move = String::new();
        let mut dx = end_pos.0 - start_pos.0;
        let mut dy = end_pos.1 - start_pos.1;
        let mut pos = start_pos.clone();

        while dx != 0 || dy != 0 {
            let temp_pos_dy = (pos.0, pos.1 + dy);
            if dy != 0 && !keyboard.emptys.contains(&temp_pos_dy) && vert == 1{
                if dy < 0 {
                    curr_move.push_str(&"^".repeat(dy.abs() as usize));
                } else {
                    curr_move.push_str(&"v".repeat(dy as usize));
                }
                dy = 0;
                pos = temp_pos_dy;
            }
            let temp_pos_dx = (pos.0 + dx, pos.1);
            if dx != 0 && !keyboard.emptys.contains(&temp_pos_dx) {
                if dx < 0 {
                    curr_move.push_str(&"<".repeat(dx.abs() as usize));
                } else {
                    curr_move.push_str(&">".repeat(dx as usize));
                }
                dx = 0;
                pos = temp_pos_dx;
            }
            let temp_pos_dy = (pos.0, pos.1 + dy);
            if dy != 0 && !keyboard.emptys.contains(&temp_pos_dy) {
                if dy < 0 {
                    curr_move.push_str(&"^".repeat(dy.abs() as usize));
                } else {
                    curr_move.push_str(&"v".repeat(dy as usize));
                }
                dy = 0;
                pos = temp_pos_dy;
            }
        }
        curr_move.push_str(&"A");
        if moves.contains(&curr_move) {
            continue;
        }
        moves.push(curr_move);
    }
    
    moves
}

fn calc_cost(from: char, to: char, level: u64, cache: &mut Cache, keyboard: &Keyboard) -> u64 {
    let mut cost = u64::MAX;

    if let Some(c) = cache.get(from, to, level) {
        return c;
    }

    let opt = generate_moves(&keyboard, from, to);

    if level == 0 {
        let val = opt.iter().map(|x| x.len() as u64).min().unwrap();
        cache.set(from, to, level, val);
        return val;
    }

    for o in opt {
        let mut last = 'A';
        let mut curr_cost = 0u64;
        for c in o.chars() {
            curr_cost += calc_cost(last, c, level-1, cache, &Keyboard::as_controll());
            last = c;
        }
        cost = std::cmp::min(cost, curr_cost);
    }

    cache.set(from, to, level, cost);
    cost
}

fn main() {
    let mut cache = Cache::new();
    let levels = 25u64;

    let mut output = 0u64;

    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines(){
        let val = line.replace("A", "").parse::<u64>().unwrap();
        let mut cost = 0;
        let mut last = 'A';
        for c in line.chars() {
            cost += calc_cost(last, c, levels, &mut cache, &Keyboard::as_numeric());
            last = c;
        }
        output += cost * val;
    }

    print!("{}", output);
}
