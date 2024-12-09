use adventofcode2024::ascii::{Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn part1() {
    println!("Part 1: {}", 1);
}

fn part2() {
    println!("Part 2: {}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day09.txt");
    let line = input.read_line();
    let mut disk: Vec<i64> = Vec::with_capacity(line.len() * 9);
    let mut empty = false;
    let mut id = 0;
    for c in line.chars() {
        let n = c.to_digit(10).unwrap();
        if empty {
            for i in 0..n {
                disk.push(-1);
            }
        } else {
            for i in 0..n {
                disk.push(id);
            }
            id += 1;
        }
        empty = !empty;
    }


    let mut i = 0;
    while i < disk.len() -1 {
        if disk[i] == -1 {
            let mut v = disk.pop().unwrap();
            while v == -1 && i < disk.len() - 1 { v = disk.pop().unwrap(); }
            disk[i] = v;
        }
        i += 1;
    }
    while *disk.last().unwrap() == -1 { disk.pop(); }

    println!("{:?}", disk);

    let sum: i64 = disk.iter().enumerate().map(|(i, v)| i as i64 * *v).sum();

    println!("{:?}", sum);

    part1();
    part2();

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
