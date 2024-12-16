use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State(Vec<Pos>, Dir, i64);

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
    work.push(State(vec![start], Dir::E, 0));
    while let Some(s) = work.pop() {
        let pos = *s.0.last().unwrap();
        let dir = s.1;
        if done.contains(&(pos, dir)) {
            continue;
        }
        done.insert((pos, dir));
        if pos == end {
            println!("Part 1: {:?}", s.2);
            return;
        }
        work.push(State(s.0.clone(), dir.right90(), s.2 + 1000));
        work.push(State(s.0.clone(), dir.left90(), s.2 + 1000));
        let target = pos.step(dir);
        if map.get(target) == '.' {
            let mut extended_path = s.0.clone();
            extended_path.push(target);
            work.push(State(extended_path, dir, s.2 + 1));
        }
    }
    panic!("No path found!");
}


fn part2(map: &Map, start: Pos, end: Pos) {
    let mut work: BinaryHeap<State> = BinaryHeap::new();
    let mut done: HashMap<(Pos, Dir), i64> = HashMap::new();
    let mut best: Vec<State> = Vec::new();
    work.push(State(vec![start], Dir::E, 0));
    while let Some(s) = work.pop() {
        let pos = *s.0.last().unwrap();
        let dir = s.1;
        if let Some(cc) = done.get(&(pos, dir)) {
            if *cc < s.2 {
                continue;
            }
        }
        done.insert((pos, dir), s.2);
        if pos == end {
            if !best.is_empty() && best.first().unwrap().2 < s.2 {
                break;
            }
            best.push(s.clone());
        }
        work.push(State(s.0.clone(), dir.right90(), s.2 + 1000));
        work.push(State(s.0.clone(), dir.left90(), s.2 + 1000));
        let target = pos.step(dir);
        if map.get(target) == '.' {
            let mut extended_path = s.0.clone();
            extended_path.push(target);
            work.push(State(extended_path, dir, s.2 + 1));
        }
    }

    let cnt: HashSet<Pos> = best.iter().map(|s| s.0.clone()).flatten().collect();
    println!("Part 2: {:?}", cnt.len());
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
