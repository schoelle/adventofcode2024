use adventofcode2024::ascii::{Dir, Pos};
use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Problem {
    startx: i64,
    starty: i64,
    velx: i64,
    vely: i64,
}

const XSIZE: i64 = 101;
const YSIZE: i64 = 103;

fn part1(problems: &Vec<Problem>) {
    let targets: &Vec<Pos> = &problems
        .iter()
        .map(|p| Pos(p.startx + 100 * p.velx, p.starty + 100 * p.vely).wrap(XSIZE, YSIZE))
        .collect();
    let mut counts = Vec::from([0i64; 4]);
    for t in targets {
        if t.0 < XSIZE / 2 && t.1 < YSIZE / 2 {
            counts[0] += 1;
        }
        if t.0 < XSIZE / 2 && t.1 > YSIZE / 2 {
            counts[1] += 1;
        }
        if t.0 > XSIZE / 2 && t.1 < YSIZE / 2 {
            counts[2] += 1;
        }
        if t.0 > XSIZE / 2 && t.1 > YSIZE / 2 {
            counts[3] += 1;
        }
    }
    println!("Part 1: {:?}", counts.iter().fold(1, |a, b| a * b));
}

fn cluster_value(robots: &HashSet<Pos>) -> i64 {
    robots.iter().filter(
        |p| robots.contains(&p.step(Dir::E)) || robots.contains(&p.step(Dir::S))
    ).count() as i64
}

fn part2(problems: &Vec<Problem>) {
    let mut i = 0;
    loop {
        let targets: &HashSet<Pos> = &problems
            .iter()
            .map(|p| Pos(p.startx + i * p.velx, p.starty + i * p.vely).wrap(XSIZE, YSIZE))
            .collect();
        if cluster_value(&targets) > 200 {
            println!("Part 2: {:?}", i);
            break;
        }
        i += 1;
    }
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day14.txt");
    let re: Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut problems: Vec<Problem> = Vec::new();
    for line in input.read_lines() {
        let cap = re.captures(&line).unwrap();
        problems.push(Problem {
            startx: cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            starty: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            velx: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            vely: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        })
    }

    part1(&problems);
    part2(&problems);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
