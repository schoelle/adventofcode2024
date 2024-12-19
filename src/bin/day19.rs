use adventofcode2024::input::AocInput;
use std::time::Instant;
use regex::Regex;


fn part1(towel_str: &str, patterns: &Vec<String>) {
    let mut exp:String = "^(".to_string();
    exp.push_str(&towel_str.replace(", ", "|"));
    exp.push_str(")+$");
    let re = Regex::new(&exp).unwrap();;

    println!("Part 1: {}", patterns.iter().filter(|p| re.is_match(p)).count())
}

fn part2() {
    println!("Part 2: {}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day19.txt");
    let towel_part = input.read_lines();
    let towel_str = towel_part.first().unwrap();
    let patterns = input.read_lines();

    part1(towel_str, &patterns);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
