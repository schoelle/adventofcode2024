use adventofcode2024::ascii::{Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn part1(table: &HashMap<char, Vec<Pos>>, map: &Map) {
    let mut res = HashSet::new();
    for v in table.values() {
        for a in v.iter() {
            for b in v.iter() {
                if *a != *b {
                    let (xd, yd) = a.sub(&b);
                    let anti = Pos(a.0 + xd, a.1 + yd);
                    if map.valid_pos(anti) {
                        res.insert(anti);
                    }
                }
            }
        }
    }
    println!("Part 1: {}", res.len());
}

fn part2(table: &HashMap<char, Vec<Pos>>, map: &Map) {
    let mut res = HashSet::new();
    for v in table.values() {
        for a in v.iter() {
            for b in v.iter() {
                if *a != *b {
                    let (xd, yd) = a.sub(&b);
                    let mut anti = Pos(a.0, a.1);
                    while map.valid_pos(anti) {
                        res.insert(anti);
                        anti = Pos(anti.0 + xd, anti.1 + yd);
                    }
                }
            }
        }
    }
    println!("Part 2: {}", res.len());
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day08.txt");
    let map = input.read_map();
    let mut table: HashMap<char, Vec<Pos>> = HashMap::new();
    map.enumerate()
        .filter(|(_, c)| *c != '.')
        .for_each(|(p, c)| {
            table.entry(c).or_insert_with(|| vec![]).push(p);
        });

    part1(&table, &map);
    part2(&table, &map);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
