use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
enum Op {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Rule {
    a: String,
    b: String,
    op: Op,
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day24.txt");
    let re1 = Regex::new("(.*): ([01])").unwrap();
    let re2 = Regex::new("(.*) (AND|OR|XOR) (.*) -> (.*)").unwrap();
    let mut init: HashMap<String, bool> = HashMap::new();
    for line in input.read_lines() {
        if let Some(c) = re1.captures(&line) {
            let varname = c.get(1).unwrap().as_str().to_string();
            let value = c.get(2).unwrap().as_str() == "1";
            init.insert(varname, value);
        }
    }
    let mut rules: HashMap<String, Rule> = HashMap::new();
    for line in input.read_lines() {
        if let Some(c) = re2.captures(&line) {
            let a = c.get(1).unwrap().as_str().to_string();
            let b = c.get(3).unwrap().as_str().to_string();
            let out = c.get(4).unwrap().as_str().to_string();
            let op = match c.get(2).unwrap().as_str() {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => panic!("Unknown operation")
            };
            rules.insert(out, Rule { a, b, op });
        }
    }

    let mut defined: Vec<String> = init.keys().cloned().collect();
    let mut todo = rules.clone();
    let mut values = init.clone();

    while !todo.is_empty() {
        let dup = todo.clone();
        let (k, r) = dup.iter().filter(
            |(_, v)| defined.contains(&v.a) && defined.contains(&v.b)
        ).next().unwrap();
        todo.remove(k);
        defined.push(k.clone());
        values.insert(k.clone(), match r.op {
            Op::AND => *values.get(&r.a).unwrap() && *values.get(&r.b).unwrap(),
            Op::OR => *values.get(&r.a).unwrap() || *values.get(&r.b).unwrap(),
            Op::XOR => *values.get(&r.a).unwrap() != *values.get(&r.b).unwrap()
        });
    }

    let mut i = 0u32;
    let mut res = 0i64;
    while let Some((_, v)) = values.get_key_value(&format!("z{:02}", i)) {
        if *v {
            res += 2i64.pow(i);
        }
        i += 1;
    }

    println!("Part 1: {}", res);


    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
