use adventofcode2024::input::AocInput;
use regex::Regex;
use std::time::Instant;

fn solves(target: i64, values: &[i64], part2: bool) -> bool {
    if values.len() == 1 && values[0] == target {
        return true;
    }
    if let Some((last, rest)) = values.split_last() {
        if target % last == 0 && solves(target / last, rest, part2) {
            return true;
        }
        if target >= *last && solves(target - last, rest, part2) {
            return true;
        }
        if part2 {
            let t_str = target.to_string();
            let l_str = last.to_string();
            if t_str.len() > l_str.len() && t_str.ends_with(l_str.as_str()) {
                let r = &t_str.as_str()[0..t_str.len() - l_str.len()];
                if solves(r.parse::<i64>().unwrap(), rest, part2) {
                    return true;
                }
            }
        }
    };
    false
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day07.txt");
    let mut part1 = 0;
    let mut part2 = 0;
    let re = Regex::new(r"(\d+): (.*)").unwrap();
    for line in input.read_lines() {
        if let Some(cap) = re.captures(&line) {
            let target = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let values: Vec<i64> = cap
                .get(2)
                .unwrap()
                .as_str()
                .split(' ')
                .map(|v| v.parse::<i64>().unwrap())
                .collect();
            if solves(target, &values, false) {
                part1 += target;
            }
            if solves(target, &values, true) {
                part2 += target;
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
