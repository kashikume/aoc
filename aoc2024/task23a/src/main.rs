use std::collections::{HashMap, HashSet};

fn add_connection(connections: &mut HashMap<String, Vec<String>>, a: &str, b: &str) {
    connections.entry(a.to_string()).or_insert(vec![]).push(b.to_string());
    connections.entry(b.to_string()).or_insert(vec![]).push(a.to_string());
}

fn add_lan(lans: &mut HashSet<Vec<String>>, lan: Vec<String>) {
    let mut sorted_lan = lan.clone();
    sorted_lan.sort();
    lans.insert(sorted_lan);
}

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut lans: HashSet<Vec<String>> = HashSet::new();

    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        add_connection(&mut connections, parts[0], parts[1]);
    }

    for (comp,conn) in connections.iter() {
        if comp.starts_with("t") {
            for c1 in 0..conn.len() {
                for c2 in c1+1..conn.len() {
                    if connections[&conn[c1]].contains(&conn[c2]) {
                        add_lan(&mut lans, vec![comp.clone(), conn[c1].clone(), conn[c2].clone()]);
                    }
                }
            }
        }
    }

    println!("There are {} LANs", lans.len());
}
