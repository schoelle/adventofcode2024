use crate::Constraint::*;
use adventofcode2024::ascii::Pos;
use adventofcode2024::input::AocInput;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Constraint {
    ANY,
    VFIRST,
    HFIRST,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    x: i64,
    y: i64,
    rule: Constraint,
}

impl Move {
    fn cost(self: &Move) -> i64 {
        self.x.abs() + self.y.abs() + 1
    }
}

fn num_pad_move(init: Pos, c: char) -> (Pos, Move) {
    match c {
        'A' => (
            Pos(2, 3),
            Move {
                x: 2 - init.0,
                y: 3 - init.1,
                rule: if init.0 == 0 { HFIRST } else { ANY },
            },
        ),
        '0' => (
            Pos(1, 3),
            Move {
                x: 1 - init.0,
                y: 3 - init.1,
                rule: if init.0 == 0 { HFIRST } else { ANY },
            },
        ),
        '1' => (
            Pos(0, 2),
            Move {
                x: 0 - init.0,
                y: 2 - init.1,
                rule: if init.1 == 3 { VFIRST } else { ANY },
            },
        ),
        '2' => (
            Pos(1, 2),
            Move {
                x: 1 - init.0,
                y: 2 - init.1,
                rule: ANY,
            },
        ),
        '3' => (
            Pos(2, 2),
            Move {
                x: 2 - init.0,
                y: 2 - init.1,
                rule: ANY,
            },
        ),
        '4' => (
            Pos(0, 1),
            Move {
                x: 0 - init.0,
                y: 1 - init.1,
                rule: if init.1 == 3 { VFIRST } else { ANY },
            },
        ),
        '5' => (
            Pos(1, 1),
            Move {
                x: 1 - init.0,
                y: 1 - init.1,
                rule: ANY,
            },
        ),
        '6' => (
            Pos(2, 1),
            Move {
                x: 2 - init.0,
                y: 1 - init.1,
                rule: ANY,
            },
        ),
        '7' => (
            Pos(0, 0),
            Move {
                x: 0 - init.0,
                y: 0 - init.1,
                rule: if init.1 == 3 { VFIRST } else { ANY },
            },
        ),
        '8' => (
            Pos(1, 0),
            Move {
                x: 1 - init.0,
                y: 0 - init.1,
                rule: ANY,
            },
        ),
        '9' => (
            Pos(2, 0),
            Move {
                x: 2 - init.0,
                y: 0 - init.1,
                rule: ANY,
            },
        ),
        _ => panic!("Unknown Char"),
    }
}

fn dir_pad_move(init: Pos, c: char) -> (Pos, Move) {
    match c {
        'A' => (
            Pos(2, 0),
            Move {
                x: 2 - init.0,
                y: 0 - init.1,
                rule: if init.0 == 1 { HFIRST } else { ANY },
            },
        ),
        '^' => (
            Pos(1, 0),
            Move {
                x: 1 - init.0,
                y: 0 - init.1,
                rule: if init.0 == 1 { HFIRST } else { ANY },
            },
        ),
        '<' => (
            Pos(0, 1),
            Move {
                x: 0 - init.0,
                y: 1 - init.1,
                rule: if init.1 == 0 { VFIRST } else { ANY },
            },
        ),
        'v' => (
            Pos(1, 1),
            Move {
                x: 1 - init.0,
                y: 1 - init.1,
                rule: ANY,
            },
        ),
        '>' => (
            Pos(2, 1),
            Move {
                x: 2 - init.0,
                y: 1 - init.1,
                rule: ANY,
            },
        ),
        _ => panic!("Unknown Char"),
    }
}

fn dir_moves_for_move(init: Pos, mve: Move) -> (Pos, Vec<Move>) {
    let mut hfirst_moves: Vec<Move> = Vec::new();
    let mut hfirst_pos: Pos = init;
    for _ in 0..mve.x.abs() {
        let c = if mve.x > 0 { '>' } else { '<' };
        let (p, m) = dir_pad_move(hfirst_pos, c);
        hfirst_pos = p;
        hfirst_moves.push(m);
    }
    for _ in 0..mve.y.abs() {
        let c = if mve.y > 0 { 'v' } else { '^' };
        let (p, m) = dir_pad_move(hfirst_pos, c);
        hfirst_pos = p;
        hfirst_moves.push(m);
    }
    let mut vfirst_moves: Vec<Move> = Vec::new();
    let mut vfirst_pos: Pos = init;
    for _ in 0..mve.y.abs() {
        let c = if mve.y > 0 { 'v' } else { '^' };
        let (p, m) = dir_pad_move(vfirst_pos, c);
        vfirst_pos = p;
        vfirst_moves.push(m);
    }
    for _ in 0..mve.x.abs() {
        let c = if mve.x > 0 { '>' } else { '<' };
        let (p, m) = dir_pad_move(vfirst_pos, c);
        vfirst_pos = p;
        vfirst_moves.push(m);
    }

    let (vp, vm) = dir_pad_move(vfirst_pos, 'A');
    vfirst_pos = vp;
    vfirst_moves.push(vm);
    let (vp, vm) = dir_pad_move(hfirst_pos, 'A');
    hfirst_pos = vp;
    hfirst_moves.push(vm);

    match mve.rule {
        VFIRST => (vfirst_pos, vfirst_moves),
        HFIRST => (hfirst_pos, hfirst_moves),
        ANY => {
            let vcost: i64 = vfirst_moves.iter().map(|m| m.cost()).sum();
            let hcost: i64 = hfirst_moves.iter().map(|m| m.cost()).sum();
            if vcost < hcost {
                (vfirst_pos, vfirst_moves)
            } else {
                (hfirst_pos, hfirst_moves)
            }
        }
    }
}

fn calc_num_moves(init: Pos, seq: &Vec<char>) -> Vec<Move> {
    let mut pos = init;
    let mut res = Vec::new();
    for s in seq {
        let (p, m) = num_pad_move(pos, *s);
        pos = p;
        res.push(m);
    }
    res
}

fn calc_dir_moves(init: Pos, seq: &Vec<Move>) -> Vec<Move> {
    let mut pos = init;
    let mut res = Vec::new();
    for s in seq {
        let (p, m) = dir_moves_for_move(pos, *s);
        pos = p;
        res.extend(m);
    }
    res
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day21.txt");

    let mut total = 0;
    for line in input.read_lines() {
        let chars: Vec<char> = line.chars().collect();
        let value: i64 = line[..line.len() - 1].parse().unwrap();
        let moves1 = calc_num_moves(Pos(2, 3), &chars);
        let moves2 = calc_dir_moves(Pos(2, 0), &moves1);
        let moves3 = calc_dir_moves(Pos(2, 0), &moves2);
        let costs: i64 = moves3.iter().map(|m| m.cost()).sum();
        println!(
            "{:?} {:?} {:?}",
            line,
            costs,
            value
        );
        total += value * costs;
    }
    println!("Part 1: {}", total);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
