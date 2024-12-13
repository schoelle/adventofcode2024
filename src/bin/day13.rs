use adventofcode2024::input::AocInput;
use regex::Regex;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Problem {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    tx: i64,
    ty: i64,
}

fn part1(problems: &Vec<Problem>) {
    let mut tokens: i64 = 0;
    for p in problems {
        let v1 = (p.tx * p.ay) - (p.ty * p.ax);
        let v2 = (p.bx * p.ay) - (p.by * p.ax);
        if v2 != 0 {
            if v1 % v2 == 0 {
                let b = v1 / v2;
                let a = (p.tx - (b * p.bx)) / p.ax;
                tokens += a * 3 + b;
            }
        } else {
            panic!("{:?}", p);
        }
    }
    println!("Part 1: {:?}", tokens);
}

fn part2(problems: &Vec<Problem>) {
    let mut tokens: u64 = 0;
    for p in problems {
        let tx = p.tx + 10000000000000;
        let ty = p.ty + 10000000000000;
        let v1 = (tx * p.ay) - (ty * p.ax);
        let v2 = (p.bx * p.ay) - (p.by * p.ax);
        if v2 != 0 {
            if v1 % v2 == 0 {
                let b = v1 / v2;
                let a = (tx - (b * p.bx)) / p.ax;
                if a * p.ax + b * p.bx == tx {
                    tokens += (a * 3 + b) as u64;
                }
            }
        } else {
            panic!("{:?}", p);
        }
    }
    println!("Part 2: {:?}", tokens);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day13.txt");
    let re1: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re2: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re3: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut problems: Vec<Problem> = Vec::new();
    while !input.is_finished() {
        let lines = input.read_lines();

        let cap1 = re1.captures(&lines[0]).unwrap();
        let cap2 = re2.captures(&lines[1]).unwrap();
        let cap3 = re3.captures(&lines[2]).unwrap();
        problems.push(Problem {
            ax: cap1.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            ay: cap1.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            bx: cap2.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            by: cap2.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            tx: cap3.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            ty: cap3.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        })
    }

    part1(&problems);
    part2(&problems);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
