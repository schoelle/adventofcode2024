use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

fn part1(towel_str: &str, patterns: &Vec<String>) {
    let mut exp: String = "^(".to_string();
    exp.push_str(&towel_str.replace(", ", "|"));
    exp.push_str(")+$");
    let re = Regex::new(&exp).unwrap();

    println!(
        "Part 1: {}",
        patterns.iter().filter(|p| re.is_match(p)).count()
    )
}

fn matches(towels: &Vec<&str>, pattern: &str, cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(x) = cache.get(pattern) {
        return *x;
    }
    let mut r = 0;
    for p in towels {
        if pattern.starts_with(p) {
            let l = p.len();
            if l == pattern.len() {
                r += 1;
            } else {
                r += matches(towels, &pattern[l..], cache);
            }
        }
    }
    cache.insert(pattern.to_string(), r);
    r
}

fn part2(towels: &Vec<&str>, patterns: &Vec<String>) {
    let mut cache: HashMap<String, i64> = HashMap::new();
    println!(
        "Part 2: {}",
        patterns
            .iter()
            .map(|p| matches(towels, p, &mut cache))
            .sum::<i64>()
    );
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day19.txt");
    let towel_part = input.read_lines();
    let towel_str = towel_part.first().unwrap();
    let patterns = input.read_lines();

    part1(towel_str, &patterns);
    part2(&towel_str.split(", ").collect(), &patterns);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
