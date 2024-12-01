use adventofcode2024::input::AocInput;
use regex::Regex;

fn part1(input: &mut AocInput) {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut res = 0;
    for line in input.read_lines() {
        for exp in re.captures_iter(&line) {
            let a = exp.name("a").unwrap().as_str().parse::<i64>().unwrap();
            let b = exp.name("b").unwrap().as_str().parse::<i64>().unwrap();
            res += a * b;
        }
    }
    println!("Part 1: {}", res);
}

fn part2(input: &mut AocInput) {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut res = 0;
    let mut enabled = true;
    for line in input.read_lines() {
        for exp in re.captures_iter(&line) {
            if exp.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if exp.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            } else if enabled{
                let a = exp.name("a").unwrap().as_str().parse::<i64>().unwrap();
                let b = exp.name("b").unwrap().as_str().parse::<i64>().unwrap();
                res += a * b;
            }

        }
    }
    println!("Part 1: {}", res);
}

fn main() {
    let mut input = AocInput::new("inputs/day03.txt");
    part1(&mut input);
    input.reset();
    part2(&mut input);
}