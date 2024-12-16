use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State(Pos, Dir, i64, i64);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.3.cmp(&self.3)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dist(a: Pos, b: Pos) -> i64 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) * 2
}

fn part1(map: &Map, start: Pos, end: Pos) {
    let mut work: BinaryHeap<State> = BinaryHeap::new();
    let mut done: HashSet<(Pos,Dir)> = HashSet::new();
    work.push(State(start, Dir::E, 0, 0 + dist(start, end)));
    while let Some(s) = work.pop() {
        if done.contains(&(s.0, s.1)) {
            continue;
        }
        // let mut cmap = map.clone();
        // cmap.set(s.0, s.1.to_char());
        // println!("{}", cmap.to_string());
        // println!("{:?}", s);

        if s.0 == end {
            println!("Part 1: {:?}", s.2);
            return;
        }
        work.push(State(
            s.0,
            s.1.right90(),
            s.2 + 1000,
            s.2 + 1000 + dist(s.0, end),
        ));
        work.push(State(
            s.0,
            s.1.left90(),
            s.2 + 1000,
            s.2 + 1000 + dist(s.0, end),
        ));
        let t = s.0.step(s.1);
        if map.get(t) == '.' {
            work.push(State(t, s.1, s.2 + 1, s.2 + 1 + dist(s.0, end)));
        }
        done.insert((s.0, s.1));
    }
    panic!("No path found!");
}

fn part2(_map: &Map, start: Pos, end: Pos) {
    println!("Part 2: {}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day16.txt");
    let mut map = input.read_map();
    let start_pos = *map.find('S').first().unwrap();
    let end_pos = *map.find('E').first().unwrap();
    map.set(start_pos, '.');
    map.set(end_pos, '.');
    part1(&map, start_pos, end_pos);
    part2(&map, start_pos, end_pos);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
