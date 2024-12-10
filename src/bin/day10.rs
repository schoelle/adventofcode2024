use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::HashSet;
use std::time::Instant;

fn reachable(pos: Pos, map: &Map) -> HashSet<Pos> {
    let mut todo = Vec::new();
    let mut reachable = HashSet::new();
    todo.push(pos);
    while let Some(p) = todo.pop() {
        reachable.insert(p);
        let height = map.get(p).to_digit(10).unwrap();
        for d in Vec::from([Dir::N, Dir::E, Dir::W, Dir::S]) {
            let q = p.step(d);
            let target_height = map.get(q).to_digit(10).unwrap();
            if target_height == height + 1 {
                todo.push(q);
            }
        }
    }
    reachable
}

fn part1(map: &Map) {
    let s: usize = map
        .enumerate()
        .filter(|(_, c)| *c == '0')
        .map(|(p, _)| reachable(p, &map).iter().filter(|p| map.get(**p) == '9').count())
        .sum();
    println!("Part 1: {:?}", s);
}

fn part2(map: &Map) {
    println!("Part 2: {:?}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/tst2.txt");
    let mut map = input.read_map();
    map.set_background('0');
    part1(&map);
    part2(&map);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
