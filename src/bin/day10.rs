use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::HashSet;
use std::time::Instant;

fn reachable(start: Pos, map: &Map) -> usize {
    // Iterative search
    let mut todo = Vec::new();
    let mut reachable = HashSet::new();
    todo.push(start);
    while let Some(p) = todo.pop() {
        if reachable.contains(&p) {
            continue;
        }
        reachable.insert(p);
        let height = map.get(p).to_digit(10).unwrap();
        Vec::from([Dir::N, Dir::E, Dir::W, Dir::S])
            .iter()
            .map(|d| p.step(*d))
            .filter(|p| map.get(*p).to_digit(10).unwrap() == height + 1)
            .for_each(|p| todo.push(p));
    }
    reachable.iter().filter(|p| map.get(**p) == '9').count()
}

fn paths(start: Pos, map: &Map) -> usize {
    // Recursive search
    let height = map.get(start).to_digit(10).unwrap();
    if height == 9 {
        return 1;
    }
    Vec::from([Dir::N, Dir::E, Dir::W, Dir::S])
        .iter()
        .map(|d| start.step(*d))
        .filter(|p| map.get(*p).to_digit(10).unwrap() == height + 1)
        .map(|p| paths(p, map))
        .sum()
}

fn part1(map: &Map) {
    let s: usize = map
        .enumerate()
        .filter(|(_, c)| *c == '0')
        .map(|(p, _)| reachable(p, &map))
        .sum();
    println!("Part 1: {:?}", s);
}

fn part2(map: &Map) {
    let s: usize = map
        .enumerate()
        .filter(|(_, c)| *c == '0')
        .map(|(p, _)| paths(p, &map))
        .sum();
    println!("Part 2: {:?}", s);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day10.txt");
    let mut map = input.read_map();
    map.set_background('0');
    part1(&map);
    part2(&map);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
