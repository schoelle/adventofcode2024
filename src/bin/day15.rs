use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::time::Instant;

fn gps(map: &Map, c: char) -> i64 {
    map.find(c).iter().map(|p| p.0 + 100 * p.1).sum()
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

fn can_step(m: &Map, p: Pos, d: Dir) -> bool {
    let i = m.get(p);
    let t = p.step(d);
    match i {
        '.' => true,
        '#' => false,
        '[' => {
            if d == Dir::N || d == Dir::S {
                let r = t.step(Dir::E);
                can_step(m, t, d) && can_step(m, r, d)
            } else {
                can_step(m, t.step(d), d)
            }
        }
        ']' => {
            if d == Dir::N || d == Dir::S {
                let r = t.step(Dir::W);
                can_step(m, t, d) && can_step(m, r, d)
            } else {
                can_step(m, t.step(d), d)
            }
        }
        _ => can_step(m, t, d)
    }
}

fn do_step(m: &mut Map, p: Pos, d: Dir) {
    let i = m.get(p);
    let t = p.step(d);
    match i {
        '.' => {}
        '#' => panic!("Trying to move walls"),
        '[' => {
            match d {
                Dir::N | Dir::S => {
                    do_step(m, t, d);
                    do_step(m, t.step(Dir::E), d);
                    m.set(t, '[');
                    m.set(t.step(Dir::E), ']');
                    m.set(p, '.');
                    m.set(p.step(Dir::E), '.');
                }
                Dir::E => {
                    do_step(m, t.step(d), d);
                    m.set(p.step(d).step(d), ']');
                    m.set(p.step(d), '[');
                    m.set(p, '.');
                }
                _ => panic!("Illegal move")
            }
        }
        ']' => {
            match d {
                Dir::N | Dir::S => {
                    do_step(m, t, d);
                    do_step(m, t.step(Dir::W), d);
                    m.set(t, ']');
                    m.set(t.step(Dir::W), '[');
                    m.set(p, '.');
                    m.set(p.step(Dir::W), '.');
                }
                Dir::W => {
                    do_step(m, t.step(d), d);
                    m.set(p.step(d).step(d), '[');
                    m.set(p.step(d), ']');
                    m.set(p, '.');
                }
                _ => panic!("Illegal move")
            }
        }
        _ => {
            do_step(m, t, d);
            m.set(t, '@');
            m.set(p, '.');
        }
    }
}

fn stretch(map: &Map) -> Map {
    let mut res = Map::new(map.width * 2, map.height, '?');
    for (p, c) in map.enumerate() {
        let p1 = Pos(p.0 * 2, p.1);
        let p2 = Pos(p.0 * 2 + 1, p.1);
        match c {
            '.' => {
                res.set(p1, '.');
                res.set(p2, '.');
            }
            '#' => {
                res.set(p1, '#');
                res.set(p2, '#');
            }
            'O' => {
                res.set(p1, '[');
                res.set(p2, ']');
            }
            '@' => {
                res.set(p1, '@');
                res.set(p2, '.');
            }
            _ => panic!("Unknown map content!")
        }
    }
    res
}

fn part1(input: &mut AocInput) {
    let mut state = input.read_map();
    let dirs: Vec<Dir> = input
        .read_lines()
        .iter()
        .map(|l| l.chars().map(|c| Dir::from(c)))
        .flatten()
        .collect();
    let mut robot = *state.find('@').first().unwrap();
    for d in dirs {
        if try_step(&mut state, robot, d) {
            robot = robot.step(d);
        }
    }
    println!("Part 1: {:?}", gps(&state, 'O'));
}

fn part2(input: &mut AocInput) {
    let mut state = stretch(&input.read_map());
    let dirs: Vec<Dir> = input
        .read_lines()
        .iter()
        .map(|l| l.chars().map(|c| Dir::from(c)))
        .flatten()
        .collect();
    let mut robot = *state.find('@').first().unwrap();
    for d in dirs {
        if can_step(&state, robot, d) {
            do_step(&mut state, robot, d);
            robot = robot.step(d);
        }
    }
    println!("Part 2: {}", gps(&state, '['));
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day15.txt");

    part1(&mut input);
    input.reset();
    part2(&mut input);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
