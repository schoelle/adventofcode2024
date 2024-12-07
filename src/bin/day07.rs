use adventofcode2024::input::AocInput;
use regex::Regex;
use std::time::Instant;

fn part1(input: &mut AocInput) {
    let mut res = 0;
    let re = Regex::new(r"(\d+): (.*)").unwrap();
    for line in input.read_lines() {
        if let Some(cap) = re.captures(&line) {
            let target = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let values: Vec<i64> = cap.get(2).unwrap().as_str().split(' ').map(|v| v.parse::<i64>().unwrap()).collect();
            if computes1(target, &values) {
                res += target;
            }
        }
    }
    println!("Part 1: {}", res);
}

fn computes1(target: i64, values: &[i64]) -> bool {
    if values.len() == 1 && values[0] == target {
        return true;
    }
    if let Some((last, rest)) = values.split_last() {
        if target % last == 0 && computes1(target / last, rest) {
            return true;
        }
        if target >= *last && computes1(target - last, rest) {
            return true;
        }
    };
    false
}


fn part2(input: &mut AocInput) {
    let mut res = 0;
    let re = Regex::new(r"(\d+): (.*)").unwrap();
    for line in input.read_lines() {
        if let Some(cap) = re.captures(&line) {
            let target = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let values: Vec<i64> = cap.get(2).unwrap().as_str().split(' ').map(|v| v.parse::<i64>().unwrap()).collect();
            if computes2(target, &values) {
                res += target;
            }
        }
    }
    println!("Part 2: {}", res);
}

fn computes2(target: i64, values: &[i64]) -> bool {
    if values.len() == 1 && values[0] == target {
        return true;
    }
    if let Some((last, rest)) = values.split_last() {
        if target % last == 0 && crate::computes2(target / last, rest) {
            return true;
        }
        if target >= *last && crate::computes2(target - last, rest) {
            return true;
        }
        let t_str = target.to_string();
        let l_str = last.to_string();
        if t_str.len() > l_str.len() && t_str.ends_with(l_str.as_str()) {
            let r = &t_str.as_str()[0..t_str.len()-l_str.len()];
            if computes2(r.parse::<i64>().unwrap(), rest) {
                return true;
            }
        }
    };
    false
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day07.txt");
    part1(&mut input);
    input.reset();
    part2(&mut input);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
