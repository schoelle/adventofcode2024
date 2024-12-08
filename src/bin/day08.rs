use std::collections::{HashMap, HashSet};
use adventofcode2024::input::AocInput;
use std::time::Instant;
use adventofcode2024::ascii::{Map, Pos};

fn antinodes(v: &Vec<Pos>, map: &Map) -> HashSet<Pos> {
    let mut res = HashSet::new();
    for a in v.iter() {
        for b in v.iter() {
            if *a != *b {
                let (xd,yd) = a.sub(&b);
                let anti1 = Pos(a.0+xd, a.1+yd);
                let anti2 = Pos(b.0-xd, b.1-yd);
                if map.valid_pos(anti1) {
                    res.insert(anti1);
                }
                if map.valid_pos(anti2) {
                    res.insert(anti2);
                }
            }
        }
    }
    res
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day08.txt");
    let map = input.read_map();
    let mut table: HashMap<char, Vec<Pos>> = HashMap::new();
    for (p, c) in map.enumerate().filter(|(p,c)| *c != '.') {
        table.entry(c).or_insert_with(|| vec![]).push(p);
    }
    
    let mut res: HashSet<Pos> = HashSet::new();
    for s in table.values().map(|v| antinodes(v, &map)) {
        res.extend(s);
    };
    println!("Part 1: {}", res.len());
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
