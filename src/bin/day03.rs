use std::time::Instant;
use adventofcode2024::input::AocInput;
use regex::Regex;

fn part1(input: &mut AocInput) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    for line in input.read_lines() {
        for exp in re.captures_iter(&line) {
            let a = exp.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let b = exp.get(2).unwrap().as_str().parse::<i64>().unwrap();
            res += a * b;
        }
    }
    println!("Part 1: {}", res);
}

fn part2(input: &mut AocInput) {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;
    let mut enabled = true;
    for line in input.read_lines() {
        for exp in re.captures_iter(&line) {
            match exp.get(0).unwrap().as_str() {
                "do()" => { enabled = true}
                "don't()" => { enabled = false}
                _ => {
                    if enabled {
                        let a = exp.get(1).unwrap().as_str().parse::<i64>().unwrap();
                        let b = exp.get(2).unwrap().as_str().parse::<i64>().unwrap();
                        res += a * b;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", res);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day03.txt");
    part1(&mut input);
    input.reset();
    part2(&mut input);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}