use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::HashSet;
use std::time::Instant;

fn part1(map: &Map, start: Pos) -> HashSet<Pos> {
    let mut pos = start;
    let mut dir = Dir::N;
    let mut visited: HashSet<Pos> = HashSet::from([pos]);
    while map.get(pos) != ' ' {
        visited.insert(pos);
        if map.get(pos.step(dir)) == '#' {
            dir = dir.right90();
        } else {
            pos = pos.step(dir);
        }
    }
    println!("Part 1: {:?}", visited.len());
    visited.remove(&start);
    visited
}

fn loops(map: &Map, start: Pos, obstruction: Pos) -> bool {
    let mut pos = start;
    let mut dir = Dir::N;
    let mut state_history: HashSet<(Pos, Dir)> = HashSet::new();
    while map.get(pos) != ' ' {
        if state_history.contains(&(pos, dir)) {
            return true;
        }
        state_history.insert((pos, dir));
        let next = pos.step(dir);
        if next == obstruction || map.get(next) == '#' {
            dir = dir.right90();
        } else {
            pos = pos.step(dir);
        }
    }
    false
}

fn part2(map: &Map, visited: &HashSet<Pos>, start: Pos) {
    let cnt = visited.iter().filter(|p| loops(map, start, **p)).count();
    println!("Part 2: {}", cnt);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day06.txt");
    let map = input.read_map();
    let start_pos = map.find('^')[0];
    let visited = part1(&map, start_pos);
    part2(&map, &visited, start_pos);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
