use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

fn gps(map: &Map) -> i64 {
    map.find('O').iter().map(|p| p.0 + 100 * p.1).sum()
}

fn try_step(m: &mut Map, p: Pos, d: Dir) -> bool {
    let i = m.get(p);
    if i == '.' {
        return true;
    }
    if i == '#' {
        return false;
    }
    let t = p.step(d);
    if try_step(m, t, d) {
        m.set(t, m.get(p));
        m.set(p, '.');
        return true;
    }
    false
}

fn part1(input: &mut AocInput) {
    let mut state = input.read_map();
    let dirs: Vec<Dir> = input.read_lines().iter().map(|l| l.chars().map(|c| Dir::from(c))).flatten().collect();
    let mut robot = *state.find('@').first().unwrap();
    println!("{}", state.to_string());
    for d in dirs {
        if try_step(&mut state, robot, d) {
            robot = robot.step(d);
        }
        println!("{}", state.to_string());
    }

    println!("Part 1: {:?}", gps(&state));
}

fn part2() {
    println!("Part 2: {:?}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day15.txt");

    part1(&mut input);
    part2();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
