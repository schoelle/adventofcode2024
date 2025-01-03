use adventofcode2024::input::AocInput;
use std::time::Instant;

fn solves(target: i64, values: &[i64], part2: bool) -> bool {
    if values.len() == 1 {
        return values[0] == target;
    }
    let (last, rest) = values.split_last().unwrap();
    if target % last == 0 && solves(target / last, rest, part2) {
        return true;
    }
    if target >= *last && solves(target - last, rest, part2) {
        return true;
    }
    if part2 {
        let mask = 10i64.pow(last.ilog10() + 1);
        if target % mask == *last {
            if solves(target / mask, rest, part2) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day07.txt");
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.read_lines() {
        let (front, rest) = line.split_once(": ").unwrap();
        let target: i64 = front.parse().unwrap();
        let values: Vec<i64> = rest
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if solves(target, &values, false) {
            part1 += target;
        }
        if solves(target, &values, true) {
            part2 += target;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
