use std::collections::{HashMap, HashSet};

fn add_connection(connections: &mut HashMap<String, Vec<String>>, a: &str, b: &str) {
    connections.entry(a.to_string()).or_insert(vec![]).push(b.to_string());
    connections.entry(b.to_string()).or_insert(vec![]).push(a.to_string());
}

fn add_lan(lans: &mut HashSet<Vec<String>>, mut lan: Vec<String>) {
    lan.sort();
    lans.insert(lan);
}

fn main() {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut lans: HashSet<Vec<String>> = HashSet::new();

    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("-").collect();
        add_connection(&mut connections, parts[0], parts[1]);
    }

    println!("There are {} computers", connections.len());
    let mut max_connections = 0;
    for (comp,conn) in connections.iter() {
        max_connections = std::cmp::max(max_connections, conn.len());
    }
    println!("The maximum number of connections is {}", max_connections);

    for (comp,conn) in connections.iter() {
        for c1 in 0..conn.len() {
            for c2 in c1+1..conn.len() {
                if connections[&conn[c1]].contains(&conn[c2]) {
                    add_lan(&mut lans, vec![comp.clone(), conn[c1].clone(), conn[c2].clone()]);
                }
            }
        }
    }

    let mut found = true;
    while found{
        found = false;
        let mut new_lans: HashSet<Vec<String>> = HashSet::new();
        for (comp, conn) in connections.iter() {
            for lan in lans.iter() {
                if !lan.contains(comp) {
                    let mut good = true;
                    for c in lan.iter() {
                        if !conn.contains(c) {
                            good = false;
                            break;
                        }
                    }
                    if good {
                        found = true;
                        let mut new_lan = lan.clone();
                        new_lan.push(comp.clone());
                        add_lan(&mut new_lans, new_lan);
                    }
                }
            }
        }
        if found {
            lans = new_lans;
            println!("There are {} LANs", lans.len());
        }
    }

    for lan in lans.iter() {
        for (p, c) in lan.iter().enumerate() {
            if p > 0 {
                print!(",");
            }
            print!("{}", c);
        }
    }
}
