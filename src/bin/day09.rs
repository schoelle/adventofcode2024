use adventofcode2024::input::AocInput;
use std::time::Instant;

fn part1(line: &str) {
    let mut disk: Vec<i64> = Vec::with_capacity(line.len() * 9);
    let mut empty = false;
    let mut id = 0;
    for c in line.chars() {
        let n = c.to_digit(10).unwrap();
        if empty {
            (0..n).for_each(|_| disk.push(-1));
        } else {
            (0..n).for_each(|_| disk.push(id));
            id += 1;
        }
        empty = !empty;
    }
    let mut i = 0;
    while i < disk.len() - 1 {
        if disk[i] == -1 {
            let mut v = disk.pop().unwrap();
            while v == -1 && i < disk.len() - 1 {
                v = disk.pop().unwrap();
            }
            disk[i] = v;
        }
        i += 1;
    }
    while *disk.last().unwrap() == -1 {
        disk.pop();
    }
    let sum: i64 = disk.iter().enumerate().map(|(i, v)| i as i64 * *v).sum();
    println!("Part 1: {}", sum);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block(i64, u32);

fn part2(line: &str) {
    let mut disk: Vec<Block> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(cnt, size)| {
            if cnt % 2 == 1 {
                Block(-1, size)
            } else {
                Block(cnt as i64 / 2, size)
            }
        })
        .collect();
    for id in (0..(line.len() as i64 + 1) / 2).rev() {
        let file_pos = disk.iter().rposition(|b| b.0 == id).unwrap();
        let file = disk[file_pos];
        if let Some(empty_pos) = disk.iter().position(|b| b.0 == -1 && b.1 >= file.1) {
            if empty_pos > file_pos {
                continue;
            }
            let empty_block = disk[empty_pos];
            disk[file_pos] = Block(-1, file.1);
            if empty_block.1 == file.1 {
                disk[empty_pos] = file;
            } else {
                disk[empty_pos] = Block(-1, empty_block.1 - file.1);
                disk.insert(empty_pos, file);
            }
        }
    }
    let mut idx = 0;
    let mut sum: i64 = 0;
    for b in &disk {
        let size = b.1 as i64;
        let value: i64 = (idx..idx + size).map(|v| v * b.0).sum();
        if b.0 != -1 {
            sum += value;
        }
        idx += size;
    }
    println!("Part 2: {:?}", sum);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day09.txt");
    let line = input.read_line();
    part1(&line);
    part2(&line);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
