use std::collections::HashSet;
use adventofcode2024::input::AocInput;
use std::time::Instant;
use adventofcode2024::ascii::{Dir, Map, Pos};

fn part1(map: &Map) {
    let mut pos = map.find('^')[0];
    let mut dir = Dir::N;
    let mut visited: HashSet<Pos> = HashSet::from([pos]);
    while map.get(pos) != ' ' {
        if map.get(pos.step(dir)) == '#' {
            dir = dir.right90();
        } else {
            pos = pos.step(dir);
            visited.insert(pos);
        }
    }
    println!("Part 1: {:?}", visited.len() - 1);
}

fn loops(map: &Map, obstruction: Pos) -> bool {
    let mut pos = map.find('^')[0];
    let mut dir = Dir::N;
    let mut state_history: HashSet<(Pos,Dir)> = HashSet::new();
    while map.get(pos) != ' ' {
        if state_history.contains(&(pos,dir)) {
            return true;
        }
        state_history.insert((pos,dir));
        let next = pos.step(dir);
        if next == obstruction || map.get(next) == '#' {
            dir = dir.right90();
        } else {
            pos = pos.step(dir);
        }
    }
    false
}

fn part2(map: &Map) {
    let mut cnt = 0;
    for y in 0..map.height {
        for x in 0.. map.width {
            let pos = Pos(x,y);
            if map.get(pos) == '.' && loops(map, pos) {
                cnt += 1;
            }
        }
    }

    println!("Part 2: {}", cnt);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day06.txt");
    let map = input.read_map();
    part1(&map);
    part2(&map);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
