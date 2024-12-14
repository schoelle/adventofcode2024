use adventofcode2024::ascii::{Dir, Pos};
use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Robot {
    startx: i64,
    starty: i64,
    velx: i64,
    vely: i64,
}

const XSIZE: i64 = 101;
const YSIZE: i64 = 103;

fn part1(robots: &Vec<Robot>) {
    let finalpos: &Vec<Pos> = &robots
        .iter()
        .map(|p| Pos(p.startx + 100 * p.velx, p.starty + 100 * p.vely).wrap(XSIZE, YSIZE))
        .collect();
    let mut counts = Vec::from([0i64; 4]);
    for pos in finalpos {
        if pos.0 < XSIZE / 2 && pos.1 < YSIZE / 2 {
            counts[0] += 1;
        }
        if pos.0 < XSIZE / 2 && pos.1 > YSIZE / 2 {
            counts[1] += 1;
        }
        if pos.0 > XSIZE / 2 && pos.1 < YSIZE / 2 {
            counts[2] += 1;
        }
        if pos.0 > XSIZE / 2 && pos.1 > YSIZE / 2 {
            counts[3] += 1;
        }
    }
    println!("Part 1: {:?}", counts.iter().fold(1, |a, b| a * b));
}

fn cluster_value(positions: &HashSet<Pos>) -> i64 {
    positions.iter().filter(
        |p| positions.contains(&p.step(Dir::E)) || positions.contains(&p.step(Dir::S))
    ).count() as i64
}

fn part2(robots: &Vec<Robot>) {
    let mut i = 0;
    loop {
        let finalpos: &HashSet<Pos> = &robots
            .iter()
            .map(|p| Pos(p.startx + i * p.velx, p.starty + i * p.vely).wrap(XSIZE, YSIZE))
            .collect();
        if cluster_value(&finalpos) > 200 {
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
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.read_lines() {
        let cap = re.captures(&line).unwrap();
        robots.push(Robot {
            startx: cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            starty: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            velx: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            vely: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        })
    }

    part1(&robots);
    part2(&robots);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
