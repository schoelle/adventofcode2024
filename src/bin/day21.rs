use adventofcode2024::ascii::Pos;
use adventofcode2024::input::AocInput;
use std::time::Instant;

fn path(from: Pos, to: Pos) -> (Vec<char>, Vec<char>) {
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
    (v1, v2)
}

fn num_pad_paths(source: Pos, code: &Vec<char>) -> Vec<Vec<char>> {
    if code.is_empty() {
        return vec![vec![]];
    }
    let target = match code.first().unwrap() {
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
        for s in num_pad_paths(target, &code[1..].to_vec()) {
            res.push([hfirst.clone(), s.clone()].concat());
        }
    } else if source.1 == 3 && target.0 == 0 {
        for s in num_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else if source.0 == target.0 || source.1 == target.1 {
        for s in num_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else {
        for s in num_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
            res.push([hfirst.clone(), s.clone()].concat());
        }
    };
    res
}

fn dir_pad_paths(source: Pos, code: &Vec<char>) -> Vec<Vec<char>> {
    if code.is_empty() {
        return vec![vec![]];
    }
    let target = match code.first().unwrap() {
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
        for s in dir_pad_paths(target, &code[1..].to_vec()) {
            res.push([hfirst.clone(), s.clone()].concat());
        }
    } else if source.1 == 0 && target.0 == 0 {
        for s in dir_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else if source.0 == target.0 || source.1 == target.1 {
        for s in dir_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
        }
    } else {
        for s in dir_pad_paths(target, &code[1..].to_vec()) {
            res.push([vfirst.clone(), s.clone()].concat());
            res.push([hfirst.clone(), s.clone()].concat());
        }
    };
    res
}

fn collect_paths(source: Pos, code: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    for c in code {
        res.extend(dir_pad_paths(source, c));
    }
    res
}

fn part1(input: &mut AocInput) {
    let mut total = 0;
    for line in input.read_lines() {
        let chars: Vec<char> = line.chars().collect();
        let value: i64 = line[..line.len() - 1].parse().unwrap();
        let s1 = num_pad_paths(Pos(2, 3), &chars);
        let s2 = collect_paths(Pos(2, 0), &s1);
        let s3 = collect_paths(Pos(2, 0), &s2);
        let costs = s3.iter().map(|p| p.len() as i64).min().unwrap();
        total += value * costs;
    }
    println!("Part 1: {}", total);
}

fn part2(input: &mut AocInput) {
    let mut total = 0;
    for line in input.read_lines() {
        let chars: Vec<char> = line.chars().collect();
        let value: i64 = line[..line.len() - 1].parse().unwrap();
        let mut s = num_pad_paths(Pos(2, 3), &chars);
        for _ in 0..3 {
            s = collect_paths(Pos(2, 0), &s)
        }
        let costs = s.iter().map(|p| p.len() as i64).min().unwrap();
        total += value * costs;
    }
    println!("Part 2: {}", total);
}


fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/tst2.txt");
    part1(&mut input);
    input.reset();
    part2(&mut input);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
