use std::time::Instant;
use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use enum_iterator::all;

fn check1(m: &Map, p: Pos, d: Dir) -> bool {
    m.get(p) == 'X' &&
        m.get(p.nstep(d, 1)) == 'M' &&
        m.get(p.nstep(d, 2)) == 'A' &&
        m.get(p.nstep(d, 3)) == 'S'
}

fn check2(m: &Map, p: Pos, d: Dir) -> bool {
    m.get(p) == 'A' &&
        m.get(p.nstep(d, 1)) == 'M' &&
        m.get(p.nstep(d.left90(), 1)) == 'M' &&
        m.get(p.nstep(d.rev(), 1)) == 'S' &&
        m.get(p.nstep(d.right90(), 1)) == 'S'
}

fn part1(m: &Map) {
    let mut cnt = 0;
    for y in 0..m.height {
        for x in 0..m.width {
            cnt += all::<Dir>().into_iter().filter(|d| check1(&m, Pos(x, y), *d)).count()
        }
    }
    println!("Part 1: {}", cnt);
}

fn part2(m: &Map) {
    let mut cnt = 0;
    for y in 0..m.height {
        for x in 0..m.width {
            cnt += vec![Dir::NE, Dir::NW, Dir::SE, Dir::SW].into_iter().filter(|d| check2(&m, Pos(x, y), *d)).count()
        }
    }
    println!("Part 2: {}", cnt);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day04.txt");
    let map = input.read_map();
    part1(&map);
    part2(&map);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
