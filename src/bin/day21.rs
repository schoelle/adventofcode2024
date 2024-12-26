use adventofcode2024::ascii::Pos;
use adventofcode2024::input::AocInput;
use std::time::Instant;
use regex::Regex;

fn path(from: Pos, to: Pos) -> (String, String) {
    let xd = (from.0 - to.0).abs() as usize;
    let yd = (from.1 - to.1).abs() as usize;
    let (xc, yc) = if from.0 > to.0 {
        if from.1 > to.1 {
            ('<', '^')
        } else {
            ('<', 'v')
        }
    } else {
        if from.1 > to.1 {
            ('>', '^')
        } else {
            ('>', 'v')
        }
    };
    let v1 = [vec![xc; xd], vec![yc; yd], vec!['A']].concat();
    let v2 = [vec![yc; yd], vec![xc; xd], vec!['A']].concat();
    (v1.iter().collect(), v2.iter().collect())
}

fn num_pad_paths(source: Pos, code: &str) -> Vec<String> {
    if code.is_empty() {
        return vec!["".to_string()];
    }
    let target = match code.chars().next().unwrap() {
        'A' => Pos(2, 3),
        '0' => Pos(1, 3),
        '1' => Pos(0, 2),
        '2' => Pos(1, 2),
        '3' => Pos(2, 2),
        '4' => Pos(0, 1),
        '5' => Pos(1, 1),
        '6' => Pos(2, 1),
        '7' => Pos(0, 0),
        '8' => Pos(1, 0),
        '9' => Pos(2, 0),
        _ => panic!("Illegal Char"),
    };
    let (hfirst, vfirst) = path(source, target);
    let mut res = Vec::new();
    if source.0 == 0 && target.1 == 3 {
        for s in num_pad_paths(target, &code[1..]) {
            res.push([hfirst.clone(), s.clone()].concat());
        }
    } else if source.1 == 3 && target.0 == 0 {
        for s in num_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else if source.0 == target.0 || source.1 == target.1 {
        for s in num_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else {
        for s in num_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
            res.push([hfirst.clone(), s.clone()].concat());
        }
    };
    res
}

fn dir_pad_paths(source: Pos, code: &str) -> Vec<String> {
    if code.is_empty() {
        return vec!["".to_string()];
    }
    let target = match code.chars().next().unwrap() {
        'A' => Pos(2, 0),
        '^' => Pos(1, 0),
        '<' => Pos(0, 1),
        'v' => Pos(1, 1),
        '>' => Pos(2, 1),
        _ => panic!("Illegal Char"),
    };
    let (hfirst, vfirst) = path(source, target);
    let mut res = Vec::new();
    if source.0 == 0 && target.1 == 0 {
        for s in dir_pad_paths(target, &code[1..]) {
            res.push([hfirst.clone(), s.clone()].concat());
        }
    } else if source.1 == 0 && target.0 == 0 {
        for s in dir_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else if source.0 == target.0 || source.1 == target.1 {
        for s in dir_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else {
        for s in dir_pad_paths(target, &code[1..]) {
            res.push([vfirst.clone(), s.clone()].concat());
            res.push([hfirst.clone(), s.clone()].concat());
        }
    };
    res
}

fn split(code: &str) -> Vec<String> {
    let re = Regex::new("[v<>^]*A").unwrap();
    re.find_iter(code).into_iter().map(|m| m.as_str().to_string()).collect()
}

fn dir_for_dir(code: &str, depth: i64) -> String {
    if depth == 0 {
        return code.to_string();
    }
    let parts = split(code);
    let mut res: Vec<String> = Vec::new();
    for part in parts {
        let options = dir_pad_paths(Pos(2,0), &part);
        res.push(options.iter().next().unwrap().clone());
    }
    let s: String = res.into_iter().map(|s| s.to_string()).collect();
    dir_for_dir(&s, depth-1)
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/tst2.txt");

    for line in input.read_lines() {
        let direct = num_pad_paths(Pos(2, 3), &line);
        println!("Direct: {:?}", direct);
        for d in direct {
            let value: usize = line[..line.len() - 1].parse().unwrap();
            let dd = dir_for_dir(&d, 2);
            println!("  Indirect: {:?} {:?}", dd, dd.len() * value);
        }
    }

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
