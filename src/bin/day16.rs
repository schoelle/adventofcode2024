use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State(Pos, Dir, i64);

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(map: &Map, start: Pos, end: Pos) {
    let mut work: BinaryHeap<State> = BinaryHeap::new();
    let mut done: HashSet<(Pos, Dir)> = HashSet::new();
    work.push(State(start, Dir::E, 0));
    while let Some(s) = work.pop() {
        if done.contains(&(s.0, s.1)) {
            continue;
        }
        if s.0 == end {
            println!("Part 1: {:?}", s.2);
            return;
        }
        work.push(State(s.0, s.1.right90(), s.2 + 1000));
        work.push(State(s.0, s.1.left90(), s.2 + 1000));
        let t = s.0.step(s.1);
        if map.get(t) == '.' {
            work.push(State(t, s.1, s.2 + 1));
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
